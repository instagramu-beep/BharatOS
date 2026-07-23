# BharatOS API Reference

## Kernel API

### Process Management

```rust
pub fn init() -> Result<&'static mut Process>
pub fn create_process(ppid: Pid) -> &'static mut Process
pub fn fork(parent: &Process) -> Result<&'static mut Process>
pub fn exec(proc: &mut Process, path: &str, argv: &[&str], envp: &[&str]) -> Result<()>
pub fn exit(proc: &mut Process, code: i32)
pub fn wait(pid: Pid) -> Option<&'static mut Process>
pub fn get_process(pid: Pid) -> Option<&'static Process>
pub fn current_process() -> Option<&'static mut Process>
pub fn kill(pid: Pid, sig: Signal) -> Result<()>
pub fn signal(pid: Pid, sig: Signal, handler: fn()) -> Result<()>
pub fn set_priority(pid: Pid, prio: u8) -> Result<()>
pub fn set_affinity(pid: Pid, mask: u64) -> Result<()>
```

### Memory Management

```rust
pub fn frame_alloc() -> u64
pub fn frame_free(addr: u64)
pub fn init_kernel_heap()
pub fn kmalloc(size: usize) -> *mut u8
pub fn kfree(ptr: *mut u8)
pub fn map_page(phys: u64, virt: u64, flags: PageFlags)
pub fn unmap_page(virt: u64)
pub fn protect_memory(addr: u64, len: u64, flags: PageFlags)
pub fn swap_out(addr: u64) -> Result<SwapEntry>
pub fn swap_in(entry: SwapEntry) -> Result<()>
```

### Filesystem

```rust
pub fn init()  // Initialize VFS
pub fn mount_root(fs: &'static dyn VfsFilesystem)
pub fn mount_procfs(path: &str) -> Result<()>
pub fn mount_devfs(path: &str) -> Result<()>
pub fn mount_tmpfs(path: &str) -> Result<()>
pub trait VfsFilesystem {
    fn root(&self) -> &'static VfsNode;
    fn name(&self) -> &str;
}
pub trait VfsFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn seek(&mut self, off: u64) -> Result<u64>;
    fn flush(&mut self) -> Result<()>;
    fn close(self: Box<Self>) -> Result<()>;
}
```

### BharatFS API

```rust
pub fn mount(dev: &'static dyn BlockDevice) -> Result<&'static BharatFSFileSystem>
pub fn create(path: &str) -> Result<InodeHandle>
pub fn open(path: &str) -> Result<InodeHandle>
pub fn read(handle: InodeHandle, buf: &mut [u8]) -> Result<usize>
pub fn write(handle: InodeHandle, buf: &[u8]) -> Result<usize>
pub fn seek(handle: InodeHandle, pos: u64) -> Result<u64>
pub fn close(handle: InodeHandle) -> Result<()>
pub fn mkdir(path: &str) -> Result<()>
pub fn rmdir(path: &str) -> Result<()>
pub fn unlink(path: &str) -> Result<()>
pub fn snapshot(path: &str, name: &str) -> Result<u64>
pub fn list_snapshots(path: &str) -> Result<Vec<Snapshot>>
pub fn restore_snapshot(path: &str, snap_idx: u64) -> Result<()>
pub fn stat(path: &str) -> Result<FileStat>
pub fn chmod(path: &str, mode: u32) -> Result<()>
pub fn chown(path: &str, uid: u32, gid: u32) -> Result<()>
```

### IPC

```rust
pub fn pipe(fds: &mut [Fd; 2]) -> Result<()>
pub fn socket(domain: u32, ty: u32, protocol: u32) -> Result<Fd>
pub fn bind(sockfd: Fd, addr: &SockAddr, len: u32) -> Result<()>
pub fn listen(sockfd: Fd, backlog: i32) -> Result<()>
pub fn accept(sockfd: Fd) -> Result<(Fd, SockAddr)>
pub fn connect(sockfd: Fd, addr: &SockAddr, len: u32) -> Result<()>
pub fn mmap(addr: u64, len: u64, prot: u32, flags: u32, fd: Fd, off: u64) -> Result<u64>
pub fn shm_create(key: u64, size: u64) -> Result<u64>
pub fn shm_open(key: u64, size: u64) -> Result<u64>
pub fn msg_queue_create(key: u64, max_msg: u32, max_size: u32) -> Result<u64>
pub fn msg_send(qid: u64, msg: &[u8]) -> Result<()>
pub fn msg_recv(qid: u64, buf: &mut [u8]) -> Result<usize>
```

### Security

```rust
pub fn init() -> &'static mut SecurityManager
pub fn add_rule(rule: FirewallRule)
pub fn audit(event: SecurityEvent)
pub fn enforce_app_sandbox(app: AppId) -> SandboxPolicy
pub fn verify_file_integrity(path: &str, hash: &[u8; 32]) -> bool
pub fn cap_grant(pid: Pid, capabilities: CapSet) -> Result<()>
pub fn cap_revoke(pid: Pid, capabilities: CapSet) -> Result<()>
pub fn namespace_create(ty: NamespaceType) -> Result<NamespaceId>
pub fn namespace_enter(nsid: NamespaceId) -> Result<()>
```

## Desktop API (libsurface)

```rust
pub fn create_window(title: &str, size: (u32, u32)) -> WindowHandle
pub fn create_surface(window: WindowHandle) -> SurfaceHandle
pub fn draw_rect(ctx: &mut PaintContext, x: i32, y: i32, w: u32, h: u32, color: u32)
pub fn draw_text(ctx: &mut PaintContext, x: i32, y: i32, text: &str, size: u8, color: u32)
pub fn draw_image(ctx: &mut PaintContext, x: i32, y: i32, img: &GpuImage)
pub fn set_theme(theme: Theme) -> Result<()>
pub fn register_widget(widget: Box<dyn Widget>) -> WidgetId
pub fn send_event(event: WindowEvent) -> Result<()>
pub fn set_vsync(enabled: bool)
pub fn present(swapchain: &mut Swapchain) -> Result<()>
```

## AI API (libaep)

```rust
pub struct BharatAI {
    pub pipeline: Pipeline,
    pub model: Option<Model>,
}

impl BharatAI {
    pub fn new() -> Result<Self>;
    pub fn load_model(&mut self, path: &str) -> Result<()>;
    pub fn infer(&self, input: &Tensor) -> Result<Tensor>;
    pub fn stream_infer(&self, input: &Tensor) -> Result<Stream>;
    pub fn unload_model(&mut self);
}

pub fn text_generate(prompt: &str, max_tokens: u32) -> Result<String>
pub fn embed(text: &str) -> Result<Tensor>
pub fn classify(text: &str, classes: &[&str]) -> Result<Classification>
pub fn translate(text: &str, from: &str, to: &str) -> Result<String>
pub fn summarize(text: &str, max_len: u32) -> Result<String>
pub fn generate_code(prompt: &str, lang: &str) -> Result<String>
```

## Network API

```rust
pub fn socket(domain: u32, ty: u32, protocol: u32) -> Result<Fd>
pub fn bind(sockfd: Fd, addr: &SockAddr) -> Result<()>
pub fn listen(sockfd: Fd, backlog: i32) -> Result<()>
pub fn accept(sockfd: Fd) -> Result<(Fd, SockAddr)>
pub fn connect(sockfd: Fd, addr: &SockAddr) -> Result<()>
pub fn send(sockfd: Fd, buf: &[u8]) -> Result<usize>
pub fn recv(sockfd: Fd, buf: &mut [u8]) -> Result<usize>
pub fn getaddrinfo(host: &str) -> Result<IpAddr>
pub fn tls_connect(sockfd: Fd, sni: &str) -> Result<TlsStream>
pub fn http_get(url: &str) -> Result<HttpResponse>
pub fn websocket_connect(url: &str) -> Result<WebSocket>
```
