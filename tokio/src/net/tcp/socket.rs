use crate::net::{TcpListener, TcpStream};

use std::fmt;
use std::io;
use std::net::SocketAddr;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket};

cfg_net! {
    /// A TCP socket that has not yet been converted to a `TcpStream` or
    /// `TcpListener`.
    ///
    /// `TcpSocket` wraps an operating system socket and enables the caller to
    /// configure the socket before establishing a TCP connection or accepting
    /// inbound connections. The caller is able to set socket option and explicitly
    /// bind the socket with a socket address.
    ///
    /// The underlying socket is closed when the `TcpSocket` value is dropped.
    ///
    /// `TcpSocket` should only be used directly if the default configuration used
    /// by `TcpStream::connect` and `TcpListener::bind` does not meet the required
    /// use case.
    ///
    /// Calling `TcpStream::connect("127.0.0.1:8080")` is equivalent to:
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     let stream = socket.connect(addr).await?;
    /// # drop(stream);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Calling `TcpListener::bind("127.0.0.1:8080")` is equivalent to:
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     // On platforms with Berkeley-derived sockets, this allows to quickly
    ///     // rebind a socket, without needing to wait for the OS to clean up the
    ///     // previous one.
    ///     //
    ///     // On Windows, this allows rebinding sockets which are actively in use,
    ///     // which allows “socket hijacking”, so we explicitly don't set it here.
    ///     // https://docs.microsoft.com/en-us/windows/win32/winsock/using-so-reuseaddr-and-so-exclusiveaddruse
    ///     socket.set_reuseaddr(true)?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    /// # drop(listener);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Setting socket options not explicitly provided by `TcpSocket` may be done by
    /// accessing the `RawFd`/`RawSocket` using [`AsRawFd`]/[`AsRawSocket`] and
    /// setting the option with a crate like [`socket2`].
    ///
    /// [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
    /// [`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
    /// [`AsRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html
    /// [`AsRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawSocket.html
    /// [`socket2`]: https://docs.rs/socket2/
    #[cfg_attr(docsrs, doc(alias = "connect_std"))]
    pub struct TcpSocket {
        inner: mio::net::TcpSocket,
    }
}

impl TcpSocket {
    /// Creates a new socket configured for IPv4.
    ///
    /// Calls `socket(2)` with `AF_INET` and `SOCK_STREAM`.
    ///
    /// # Returns
    ///
    /// On success, the newly created `TcpSocket` is returned. If an error is
    /// encountered, it is returned instead.
    ///
    /// # Examples
    ///
    /// Create a new IPv4 socket and start listening.
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(128)?;
    /// # drop(listener);
    ///     Ok(())
    /// }
    /// ```
    pub fn new_v4() -> io::Result<TcpSocket> {
        let inner = mio::net::TcpSocket::new_v4()?;
        Ok(TcpSocket { inner })
    }

    /// Creates a new socket configured for IPv6.
    ///
    /// Calls `socket(2)` with `AF_INET6` and `SOCK_STREAM`.
    ///
    /// # Returns
    ///
    /// On success, the newly created `TcpSocket` is returned. If an error is
    /// encountered, it is returned instead.
    ///
    /// # Examples
    ///
    /// Create a new IPv6 socket and start listening.
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "[::1]:8080".parse().unwrap();
    ///     let socket = TcpSocket::new_v6()?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(128)?;
    /// # drop(listener);
    ///     Ok(())
    /// }
    /// ```
    pub fn new_v6() -> io::Result<TcpSocket> {
        let inner = mio::net::TcpSocket::new_v6()?;
        Ok(TcpSocket { inner })
    }

    /// Allows the socket to bind to an in-use address.
    ///
    /// Behavior is platform specific. Refer to the target platform's
    /// documentation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.set_reuseaddr(true)?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    /// # drop(listener);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_reuseaddr(&self, reuseaddr: bool) -> io::Result<()> {
        self.inner.set_reuseaddr(reuseaddr)
    }

    /// Retrieves the value set for `SO_REUSEADDR` on this socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.set_reuseaddr(true)?;
    ///     assert!(socket.reuseaddr().unwrap());
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn reuseaddr(&self) -> io::Result<bool> {
        self.inner.get_reuseaddr()
    }

    /// Allows the socket to bind to an in-use port. Only available for unix systems
    /// (excluding Solaris & Illumos).
    ///
    /// Behavior is platform specific. Refer to the target platform's
    /// documentation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.set_reuseport(true)?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos")))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos"))))
    )]
    pub fn set_reuseport(&self, reuseport: bool) -> io::Result<()> {
        self.inner.set_reuseport(reuseport)
    }

    /// Allows the socket to bind to an in-use port. Only available for unix systems
    /// (excluding Solaris & Illumos).
    ///
    /// Behavior is platform specific. Refer to the target platform's
    /// documentation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.set_reuseport(true)?;
    ///     assert!(socket.reuseport().unwrap());
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos")))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos"))))
    )]
    pub fn reuseport(&self) -> io::Result<bool> {
        self.inner.get_reuseport()
    }

    /// Sets the size of the TCP send buffer on this socket.
    ///
    /// On most operating systems, this sets the `SO_SNDBUF` socket option.
    pub fn set_send_buffer_size(&self, size: u32) -> io::Result<()> {
        self.inner.set_send_buffer_size(size)
    }

    /// Returns the size of the TCP send buffer for this socket.
    ///
    /// On most operating systems, this is the value of the `SO_SNDBUF` socket
    /// option.
    ///
    /// Note that if [`set_send_buffer_size`] has been called on this socket
    /// previously, the value returned by this function may not be the same as
    /// the argument provided to `set_send_buffer_size`. This is for the
    /// following reasons:
    ///
    /// * Most operating systems have minimum and maximum allowed sizes for the
    ///   send buffer, and will clamp the provided value if it is below the
    ///   minimum or above the maximum. The minimum and maximum buffer sizes are
    ///   OS-dependent.
    /// * Linux will double the buffer size to account for internal bookkeeping
    ///   data, and returns the doubled value from `getsockopt(2)`. As per `man
    ///   7 socket`:
    ///   > Sets or gets the maximum socket send buffer in bytes. The
    ///   > kernel doubles this value (to allow space for bookkeeping
    ///   > overhead) when it is set using `setsockopt(2)`, and this doubled
    ///   > value is returned by `getsockopt(2)`.
    ///
    /// [`set_send_buffer_size`]: #method.set_send_buffer_size
    pub fn send_buffer_size(&self) -> io::Result<u32> {
        self.inner.get_send_buffer_size()
    }

    /// Sets the size of the TCP receive buffer on this socket.
    ///
    /// On most operating systems, this sets the `SO_RCVBUF` socket option.
    pub fn set_recv_buffer_size(&self, size: u32) -> io::Result<()> {
        self.inner.set_recv_buffer_size(size)
    }

    /// Returns the size of the TCP receive buffer for this socket.
    ///
    /// On most operating systems, this is the value of the `SO_RCVBUF` socket
    /// option.
    ///
    /// Note that if [`set_recv_buffer_size`] has been called on this socket
    /// previously, the value returned by this function may not be the same as
    /// the argument provided to `set_send_buffer_size`. This is for the
    /// following reasons:
    ///
    /// * Most operating systems have minimum and maximum allowed sizes for the
    ///   receive buffer, and will clamp the provided value if it is below the
    ///   minimum or above the maximum. The minimum and maximum buffer sizes are
    ///   OS-dependent.
    /// * Linux will double the buffer size to account for internal bookkeeping
    ///   data, and returns the doubled value from `getsockopt(2)`. As per `man
    ///   7 socket`:
    ///   > Sets or gets the maximum socket send buffer in bytes. The
    ///   > kernel doubles this value (to allow space for bookkeeping
    ///   > overhead) when it is set using `setsockopt(2)`, and this doubled
    ///   > value is returned by `getsockopt(2)`.
    ///
    /// [`set_recv_buffer_size`]: #method.set_recv_buffer_size
    pub fn recv_buffer_size(&self) -> io::Result<u32> {
        self.inner.get_recv_buffer_size()
    }

    /// Gets the local address of this socket.
    ///
    /// Will fail on windows if called before `bind`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.bind(addr)?;
    ///     assert_eq!(socket.local_addr().unwrap().to_string(), "127.0.0.1:8080");
    ///     let listener = socket.listen(1024)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_localaddr()
    }

    /// Binds the socket to the given address.
    ///
    /// This calls the `bind(2)` operating-system function. Behavior is
    /// platform specific. Refer to the target platform's documentation for more
    /// details.
    ///
    /// # Examples
    ///
    /// Bind a socket before listening.
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    /// # drop(listener);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn bind(&self, addr: SocketAddr) -> io::Result<()> {
        self.inner.bind(addr)
    }

    /// Establishes a TCP connection with a peer at the specified socket address.
    ///
    /// The `TcpSocket` is consumed. Once the connection is established, a
    /// connected [`TcpStream`] is returned. If the connection fails, the
    /// encountered error is returned.
    ///
    /// [`TcpStream`]: TcpStream
    ///
    /// This calls the `connect(2)` operating-system function. Behavior is
    /// platform specific. Refer to the target platform's documentation for more
    /// details.
    ///
    /// # Examples
    ///
    /// Connecting to a peer.
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     let stream = socket.connect(addr).await?;
    /// # drop(stream);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(self, addr: SocketAddr) -> io::Result<TcpStream> {
        let mio = self.inner.connect(addr)?;
        TcpStream::connect_mio(mio).await
    }

    /// Converts the socket into a `TcpListener`.
    ///
    /// `backlog` defines the maximum number of pending connections are queued
    /// by the operating system at any given time. Connection are removed from
    /// the queue with [`TcpListener::accept`]. When the queue is full, the
    /// operating-system will start rejecting connections.
    ///
    /// [`TcpListener::accept`]: TcpListener::accept
    ///
    /// This calls the `listen(2)` operating-system function, marking the socket
    /// as a passive socket. Behavior is platform specific. Refer to the target
    /// platform's documentation for more details.
    ///
    /// # Examples
    ///
    /// Create a `TcpListener`.
    ///
    /// ```no_run
    /// use tokio::net::TcpSocket;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let addr = "127.0.0.1:8080".parse().unwrap();
    ///
    ///     let socket = TcpSocket::new_v4()?;
    ///     socket.bind(addr)?;
    ///
    ///     let listener = socket.listen(1024)?;
    /// # drop(listener);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn listen(self, backlog: u32) -> io::Result<TcpListener> {
        let mio = self.inner.listen(backlog)?;
        TcpListener::new(mio)
    }

    /// Converts a [`std::net::TcpStream`] into a `TcpSocket`. The provided
    /// socket must not have been connected prior to calling this function. This
    /// function is typically used together with crates such as [`socket2`] to
    /// configure socket options that are not available on `TcpSocket`.
    ///
    /// [`std::net::TcpStream`]: struct@std::net::TcpStream
    /// [`socket2`]: https://docs.rs/socket2/
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::net::TcpSocket;
    /// use socket2::{Domain, Socket, Type};
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     
    ///     let socket2_socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    ///
    ///     let socket = TcpSocket::from_std_stream(socket2_socket.into());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_std_stream(std_stream: std::net::TcpStream) -> TcpSocket {
        #[cfg(unix)]
        {
            use std::os::unix::io::{FromRawFd, IntoRawFd};

            let raw_fd = std_stream.into_raw_fd();
            unsafe { TcpSocket::from_raw_fd(raw_fd) }
        }

        #[cfg(windows)]
        {
            use std::os::windows::io::{FromRawSocket, IntoRawSocket};

            let raw_socket = std_stream.into_raw_socket();
            unsafe { TcpSocket::from_raw_socket(raw_socket) }
        }
    }
}

impl fmt::Debug for TcpSocket {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(fmt)
    }
}

#[cfg(unix)]
impl AsRawFd for TcpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(unix)]
impl FromRawFd for TcpSocket {
    /// Converts a `RawFd` to a `TcpSocket`.
    ///
    /// # Notes
    ///
    /// The caller is responsible for ensuring that the socket is in
    /// non-blocking mode.
    unsafe fn from_raw_fd(fd: RawFd) -> TcpSocket {
        let inner = mio::net::TcpSocket::from_raw_fd(fd);
        TcpSocket { inner }
    }
}

#[cfg(unix)]
impl IntoRawFd for TcpSocket {
    fn into_raw_fd(self) -> RawFd {
        self.inner.into_raw_fd()
    }
}

#[cfg(windows)]
impl IntoRawSocket for TcpSocket {
    fn into_raw_socket(self) -> RawSocket {
        self.inner.into_raw_socket()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpSocket {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpSocket {
    /// Converts a `RawSocket` to a `TcpStream`.
    ///
    /// # Notes
    ///
    /// The caller is responsible for ensuring that the socket is in
    /// non-blocking mode.
    unsafe fn from_raw_socket(socket: RawSocket) -> TcpSocket {
        let inner = mio::net::TcpSocket::from_raw_socket(socket);
        TcpSocket { inner }
    }
}
