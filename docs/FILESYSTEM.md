# BharatFS — BharatOS Filesystem

## Design Goals

- Modern features: snapshots, compression, deduplication, encryption
- High performance: parallel I/O, async metadata, SSD optimization
- Reliability: journaling, checksums, self-healing
- Scalability: large files, large volumes, millions of inodes

## On-Disk Format

### Superblock (Block 0)

```c
struct BharatFSVolume {
    uint64_t magic;          // 0x42534841524F4E ("BHARATN")
    uint32_t version;
    uint32_t flags;
    uint32_t block_size;
    uint64_t block_count;
    uint64_t inode_count;
    uint64_t journal_start;
    uint64_t journal_len;
    uint64_t root_inode;
    uint64_t reserved[48];
    uint128_t checksum;
};
```

### Inode Structure

```c
struct BharatFSInode {
    uint64_t ino;
    uint64_t size;
    uint64_t blocks[12];    // Direct blocks
    uint64_t indirect;      // Single indirect
    uint64_t double_indirect;
    uint64_t triple_indirect;
    uint64_t atime, mtime, ctime;
    uint64_t uid, gid;
    uint64_t mode;
    uint64_t flags;
    uint64_t link_count;
    uint128_t checksum;
};
```

### Journal

The journal is a circular buffer of fixed-size transactions. Each transaction contains:
- Transaction header (ID, checksum, number of operations)
- Operations: block write, inode write, bitmap update, etc.
- Commit record

On mount:
1. Replay all committed transactions
2. Roll back uncommitted transactions

### Snapshots

Snapshots are cheap because BharatFS is COW. A snapshot is just a root block pointer:
```
struct Snapshot {
    uint64_t inode;
    char name[255];
    uint128_t timestamp;
    uint64_t size;
    uint64_t root_block;
    uint32_t flags;
};
```

Snapshots are stored in a dedicated snapshot table area.

### Compression

- LZ4 for metadata (fast, low compression ratio)
- ZSTD for data (configurable compression levels)
- Transparent: application files appear uncompressed
- Per-file compression flag (set via chattr +c)

### Deduplication

- SHA-256 content hashes stored in inode extension
- Background dedup scanner identifies duplicate data
- Deduped blocks are reference-counted
- Degrades gracefully: dedup is opportunistic

### Encryption

- Per-volume (FDE) or per-file encryption
- AES-256-GCM for data, XChaCha20-Poly1305 for filenames
- Key material stored in kernel keyring (locked memory)
- TPM-backed key sealing on supported hardware

## Mount Options

| Option | Description |
|--------|-------------|
| `ro` | Mount read-only |
| `noatime` | Disable access time updates |
| `nodiratime` | Disable directory access time |
| `strictatime` | Always update access time |
| `compression=lz4` | Use LZ4 compression |
| `compression=zstd` | Use ZSTD compression |
| `dedup=on` | Enable deduplication |
| `encryption=aes256gcm` | Enable encryption |
| `snapshot` | Enable snapshot support |

## Performance

- Sequential read: ~2 GB/s (NVMe, no compression)
- Sequential write: ~1.5 GB/s (NVMe, no compression)
- Random read: ~500K IOPS
- Random write: ~300K IOPS
- Snapshot creation: ~1 μs (just a pointer update)
- Mount time: < 500 ms

## Utilities

- `mkfs.bharatfs` — format a volume
- `fsck.bharatfs` — check and repair filesystem
- `mount.bharatfs` — mount a BharatFS volume
- `bharat-cryptsetup` — manage disk encryption
- `tune.bharatfs` — tune filesystem parameters
