use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

pub fn socket_path() -> PathBuf {
    let user = std::env::var("USER").unwrap_or_else(|_| "default".into());
    std::env::temp_dir().join(format!("mdviewer-{user}.sock"))
}

/// Send a file path to a running instance. Returns `Ok(true)` if delivered.
pub fn deliver_to_running_instance(path: Option<&Path>) -> io::Result<bool> {
    let mut stream = match UnixStream::connect(socket_path()) {
        Ok(stream) => stream,
        Err(_) => return Ok(false),
    };

    let message = path
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    writeln!(stream, "{message}")?;
    stream.flush()?;
    Ok(true)
}

pub fn spawn_listener() -> io::Result<mpsc::Receiver<Option<PathBuf>>> {
    let listener = bind_listener()?;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else {
                continue;
            };

            let mut message = String::new();
            if stream.read_to_string(&mut message).is_err() {
                continue;
            }

            let path = message.trim();
            let open = if path.is_empty() {
                None
            } else {
                Some(PathBuf::from(path))
            };

            if tx.send(open).is_err() {
                break;
            }
        }

        let _ = fs::remove_file(socket_path());
    });

    Ok(rx)
}

fn bind_listener() -> io::Result<UnixListener> {
    let path = socket_path();

    match UnixListener::bind(&path) {
        Ok(listener) => Ok(listener),
        Err(err) if err.kind() == io::ErrorKind::AddrInUse => {
            if UnixStream::connect(&path).is_ok() {
                return Err(err);
            }
            let _ = fs::remove_file(&path);
            UnixListener::bind(&path)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    fn temp_socket_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("mdviewer-test-{name}.sock"))
    }

    #[test]
    fn deliver_returns_false_when_no_server() {
        let path = temp_socket_path("missing");
        let _ = fs::remove_file(&path);
        assert!(!deliver_to_running_instance(None).unwrap());
    }

    #[test]
    fn listener_receives_path_messages() {
        let socket = temp_socket_path("roundtrip");
        let _ = fs::remove_file(&socket);

        let listener = UnixListener::bind(&socket).unwrap();
        let (tx, rx) = mpsc::channel();
        let (ready_tx, ready_rx) = mpsc::channel();

        thread::spawn(move || {
            ready_tx.send(()).unwrap();
            let (mut stream, _) = listener.accept().unwrap();
            let mut message = String::new();
            stream.read_to_string(&mut message).unwrap();
            let path = message.trim();
            let open = if path.is_empty() {
                None
            } else {
                Some(PathBuf::from(path))
            };
            tx.send(open).unwrap();
        });

        ready_rx.recv_timeout(Duration::from_secs(1)).unwrap();

        let mut client = UnixStream::connect(&socket).unwrap();
        writeln!(client, "/tmp/notes.md").unwrap();
        client.flush().unwrap();
        drop(client);

        let received = rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(received, Some(PathBuf::from("/tmp/notes.md")));
    }
}
