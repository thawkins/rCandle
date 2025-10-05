# C++ to Rust Migration Guide for Candle

## Overview

This guide provides practical patterns and examples for migrating C++/Qt code from Candle to Rust for rCandle. It covers common patterns, idioms, and pitfalls.

## Table of Contents

1. [General Translation Patterns](#general-translation-patterns)
2. [Qt to Rust Mappings](#qt-to-rust-mappings)
3. [Memory Management](#memory-management)
4. [Concurrency Patterns](#concurrency-patterns)
5. [Error Handling](#error-handling)
6. [Common Pitfalls](#common-pitfalls)

---

## General Translation Patterns

### Class to Struct + Implementation

**C++ (Candle)**:
```cpp
// connection.h
class Connection : public QObject {
    Q_OBJECT
private:
    QSerialPort* port;
    QString buffer;
    
public:
    Connection(QObject* parent = nullptr);
    ~Connection();
    
    bool connect(const QString& portName);
    void disconnect();
    bool isConnected() const;
    
signals:
    void dataReceived(const QString& data);
    void errorOccurred(const QString& error);
};
```

**Rust (rCandle)**:
```rust
// connection.rs
use tokio::sync::mpsc;

pub struct Connection {
    port: Option<tokio_serial::SerialStream>,
    buffer: String,
    data_tx: mpsc::Sender<String>,
    error_tx: mpsc::Sender<String>,
}

impl Connection {
    pub fn new(data_tx: mpsc::Sender<String>, error_tx: mpsc::Sender<String>) -> Self {
        Self {
            port: None,
            buffer: String::new(),
            data_tx,
            error_tx,
        }
    }
    
    pub async fn connect(&mut self, port_name: &str) -> Result<(), ConnectionError> {
        // Implementation
    }
    
    pub async fn disconnect(&mut self) -> Result<(), ConnectionError> {
        // Implementation
    }
    
    pub fn is_connected(&self) -> bool {
        self.port.is_some()
    }
}
```

**Key Differences**:
- No inheritance in Rust (use traits instead)
- No signals/slots (use channels or callbacks)
- Explicit error handling with `Result`
- Async functions with `async/await`

### Qt Signals/Slots to Channels

**C++ (Candle)**:
```cpp
// Sender
emit dataReceived("some data");

// Receiver
connect(connection, &Connection::dataReceived, 
        this, &MyClass::handleData);
```

**Rust (rCandle)**:
```rust
// Sender
data_tx.send("some data".to_string()).await?;

// Receiver
while let Some(data) = data_rx.recv().await {
    self.handle_data(data);
}
```

**Alternative: Broadcast Channels** (for multiple subscribers):
```rust
use tokio::sync::broadcast;

// Create broadcast channel
let (tx, _rx) = broadcast::channel(100);

// Subscribe
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

// Send
tx.send(MachineStatus::Idle)?;

// Receive (multiple subscribers)
let status1 = rx1.recv().await?;
let status2 = rx2.recv().await?;
```

### Qt Properties to Struct Fields

**C++ (Candle)**:
```cpp
class MachineState : public QObject {
    Q_OBJECT
    Q_PROPERTY(double x READ x WRITE setX NOTIFY xChanged)
    
private:
    double m_x;
    
public:
    double x() const { return m_x; }
    void setX(double x) {
        if (m_x != x) {
            m_x = x;
            emit xChanged(x);
        }
    }
    
signals:
    void xChanged(double x);
};
```

**Rust (rCandle)**:
```rust
use tokio::sync::broadcast;

pub struct MachineState {
    x: f64,
    change_tx: broadcast::Sender<StateChange>,
}

impl MachineState {
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn set_x(&mut self, x: f64) {
        if (self.x - x).abs() > f64::EPSILON {
            self.x = x;
            let _ = self.change_tx.send(StateChange::XChanged(x));
        }
    }
}

#[derive(Debug, Clone)]
pub enum StateChange {
    XChanged(f64),
    YChanged(f64),
    // ... other changes
}
```

---

## Qt to Rust Mappings

### String Types

| C++ (Qt) | Rust | Notes |
|----------|------|-------|
| `QString` | `String` | Owned string |
| `QString` | `&str` | String slice (borrowed) |
| `QStringList` | `Vec<String>` | List of strings |
| `QByteArray` | `Vec<u8>` | Byte array |
| `QByteArray` | `&[u8]` | Byte slice |

**Example**:
```cpp
// C++
QString name = "test";
QByteArray data = name.toUtf8();
```

```rust
// Rust
let name = "test".to_string();  // String
let name_ref: &str = "test";    // &str
let data: Vec<u8> = name.into_bytes();
```

### Container Types

| C++ (Qt) | Rust | Notes |
|----------|------|-------|
| `QVector<T>` | `Vec<T>` | Dynamic array |
| `QList<T>` | `Vec<T>` | Usually Vec |
| `QLinkedList<T>` | `LinkedList<T>` | Rarely needed |
| `QMap<K, V>` | `BTreeMap<K, V>` | Ordered map |
| `QHash<K, V>` | `HashMap<K, V>` | Hash map |
| `QSet<T>` | `HashSet<T>` | Set |

**Example**:
```cpp
// C++
QVector<double> points;
points.append(1.0);
points.append(2.0);
```

```rust
// Rust
let mut points: Vec<f64> = Vec::new();
points.push(1.0);
points.push(2.0);

// Or with vec! macro
let points = vec![1.0, 2.0];
```

### Smart Pointers

| C++ (Qt) | Rust | Notes |
|----------|------|-------|
| `QSharedPointer<T>` | `Arc<T>` | Atomic reference counting |
| `QWeakPointer<T>` | `Weak<T>` | Weak reference |
| `QScopedPointer<T>` | `Box<T>` | Unique ownership |
| `T*` (raw) | `&T` or `&mut T` | Borrowed reference |

**Example**:
```cpp
// C++
QSharedPointer<Connection> conn = QSharedPointer<Connection>::create();
```

```rust
// Rust
use std::sync::Arc;

let conn = Arc::new(Connection::new());
let conn_clone = Arc::clone(&conn);  // Reference counting
```

### Threading

| C++ (Qt) | Rust | Notes |
|----------|------|-------|
| `QThread` | `std::thread` or `tokio::task` | Thread spawning |
| `QMutex` | `std::sync::Mutex` | Mutual exclusion |
| `QMutexLocker` | `lock()` method | RAII lock guard |
| `QThreadPool` | `tokio::task` | Task pool |

**Example**:
```cpp
// C++
QMutex mutex;
QMutexLocker locker(&mutex);
// Critical section
```

```rust
// Rust
use std::sync::Mutex;

let mutex = Mutex::new(data);
{
    let mut data = mutex.lock().unwrap();
    // Critical section
}  // Lock automatically released
```

### File I/O

| C++ (Qt) | Rust | Notes |
|----------|------|-------|
| `QFile` | `std::fs::File` | File handle |
| `QTextStream` | `BufReader`/`BufWriter` | Buffered I/O |
| `QFileInfo` | `std::fs::metadata` | File metadata |
| `QDir` | `std::fs` functions | Directory operations |

**Example**:
```cpp
// C++
QFile file("test.txt");
if (file.open(QIODevice::ReadOnly | QIODevice::Text)) {
    QTextStream in(&file);
    QString line = in.readLine();
}
```

```rust
// Rust (sync)
use std::fs::File;
use std::io::{BufRead, BufReader};

let file = File::open("test.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    // Process line
}

// Rust (async with tokio)
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

let file = File::open("test.txt").await?;
let reader = BufReader::new(file);
let mut lines = reader.lines();
while let Some(line) = lines.next_line().await? {
    // Process line
}
```

### Timers

| C++ (Qt) | Rust | Notes |
|----------|------|-------|
| `QTimer` | `tokio::time::interval` | Periodic timer |
| `QTimer::singleShot` | `tokio::time::sleep` | One-shot timer |

**Example**:
```cpp
// C++
QTimer* timer = new QTimer(this);
connect(timer, &QTimer::timeout, this, &MyClass::update);
timer->start(1000);  // 1 second
```

```rust
// Rust
use tokio::time::{interval, Duration};

let mut interval = interval(Duration::from_secs(1));
loop {
    interval.tick().await;
    self.update();
}
```

---

## Memory Management

### C++ RAII to Rust Ownership

**C++ (Candle)**:
```cpp
void processFile(const QString& filename) {
    QFile* file = new QFile(filename);
    if (file->open(QIODevice::ReadOnly)) {
        // Process file
    }
    delete file;  // Manual cleanup
}
```

**Rust (rCandle)**:
```rust
fn process_file(filename: &str) -> Result<()> {
    let file = File::open(filename)?;
    // Process file
    // Automatic cleanup when file goes out of scope
    Ok(())
}
```

### Reference Counting

**C++ (Candle)**:
```cpp
QSharedPointer<Parser> parser = QSharedPointer<Parser>::create();
QWeakPointer<Parser> weakParser = parser.toWeakRef();

// Later...
if (auto p = weakParser.toStrongRef()) {
    p->parse();
}
```

**Rust (rCandle)**:
```rust
let parser = Arc::new(Parser::new());
let weak_parser = Arc::downgrade(&parser);

// Later...
if let Some(p) = weak_parser.upgrade() {
    p.parse();
}
```

### Interior Mutability

When you need to mutate through a shared reference:

**Pattern**: `Arc<Mutex<T>>` or `Arc<RwLock<T>>`

```rust
use std::sync::{Arc, Mutex};

let state = Arc::new(Mutex::new(MachineState::new()));

// Thread 1
let state_clone = Arc::clone(&state);
tokio::spawn(async move {
    let mut s = state_clone.lock().unwrap();
    s.set_x(10.0);
});

// Thread 2
let state_clone = Arc::clone(&state);
tokio::spawn(async move {
    let s = state_clone.lock().unwrap();
    println!("x = {}", s.x());
});
```

**Read-Write Lock** (when reads are more common):
```rust
use std::sync::{Arc, RwLock};

let state = Arc::new(RwLock::new(MachineState::new()));

// Multiple readers
let s = state.read().unwrap();
println!("x = {}", s.x());

// Single writer
let mut s = state.write().unwrap();
s.set_x(10.0);
```

---

## Concurrency Patterns

### Qt's Thread Model to Tokio

**C++ (Candle)**:
```cpp
class Worker : public QObject {
    Q_OBJECT
public slots:
    void doWork() {
        // Heavy computation
        emit resultReady(result);
    }
signals:
    void resultReady(int result);
};

// Usage
Worker* worker = new Worker;
QThread* thread = new QThread;
worker->moveToThread(thread);
connect(thread, &QThread::started, worker, &Worker::doWork);
thread->start();
```

**Rust (rCandle)**:
```rust
use tokio::sync::mpsc;

async fn do_work(result_tx: mpsc::Sender<i32>) {
    // Heavy computation
    let result = 42;
    result_tx.send(result).await.unwrap();
}

// Usage
let (tx, mut rx) = mpsc::channel(10);
tokio::spawn(async move {
    do_work(tx).await;
});

// Receive result
let result = rx.recv().await.unwrap();
```

### Event Loop Integration

**C++ (Candle)**:
```cpp
int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    frmMain window;
    window.show();
    return app.exec();  // Qt event loop
}
```

**Rust (rCandle) with Iced**:
```rust
use iced::{Application, Settings};

fn main() -> iced::Result {
    RCandleApp::run(Settings::default())
}

struct RCandleApp {
    // App state
}

impl Application for RCandleApp {
    // Implementation
}
```

---

## Error Handling

### Qt Error Pattern to Rust Result

**C++ (Candle)**:
```cpp
bool parseGCode(const QString& code, QString& error) {
    if (code.isEmpty()) {
        error = "Empty G-code";
        return false;
    }
    // Parse...
    return true;
}

// Usage
QString error;
if (!parseGCode(code, error)) {
    qWarning() << "Parse error:" << error;
}
```

**Rust (rCandle)**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Empty G-code")]
    EmptyCode,
    #[error("Invalid command at line {line}: {message}")]
    InvalidCommand { line: usize, message: String },
}

fn parse_gcode(code: &str) -> Result<Vec<Command>, ParseError> {
    if code.is_empty() {
        return Err(ParseError::EmptyCode);
    }
    // Parse...
    Ok(commands)
}

// Usage
match parse_gcode(code) {
    Ok(commands) => {
        // Success
    }
    Err(e) => {
        tracing::error!("Parse error: {}", e);
    }
}

// Or with ? operator
let commands = parse_gcode(code)?;
```

### Exception Handling

**C++ (Candle)**:
```cpp
try {
    riskyOperation();
} catch (const std::exception& e) {
    qWarning() << "Error:" << e.what();
}
```

**Rust (rCandle)**:
```rust
// Rust doesn't use exceptions; use Result instead
match risky_operation() {
    Ok(value) => {
        // Success
    }
    Err(e) => {
        tracing::error!("Error: {}", e);
    }
}

// Or propagate with ?
let value = risky_operation()?;
```

**Panic** (similar to uncaught exception - should be rare):
```rust
// Only for unrecoverable errors
if critical_invariant_violated {
    panic!("Critical error: invariant violated");
}

// Better: return Result
if critical_invariant_violated {
    return Err(Error::InvariantViolated);
}
```

---

## Common Pitfalls

### 1. Ownership and Borrowing

**Problem**: Trying to use value after move
```rust
// ❌ Wrong
let s = String::from("hello");
process(s);  // s moved here
println!("{}", s);  // ❌ Error: value borrowed after move
```

**Solution**: Use references or clone
```rust
// ✅ Correct - borrow
let s = String::from("hello");
process(&s);  // Borrow s
println!("{}", s);  // OK

// ✅ Correct - clone
let s = String::from("hello");
process(s.clone());  // Clone s
println!("{}", s);  // OK
```

### 2. Mutable References

**Problem**: Multiple mutable references
```rust
// ❌ Wrong
let mut v = vec![1, 2, 3];
let r1 = &mut v;
let r2 = &mut v;  // ❌ Error: cannot borrow as mutable more than once
r1.push(4);
```

**Solution**: Only one mutable reference at a time
```rust
// ✅ Correct
let mut v = vec![1, 2, 3];
{
    let r1 = &mut v;
    r1.push(4);
}  // r1 scope ends
let r2 = &mut v;  // OK now
r2.push(5);
```

### 3. Lifetime Issues

**Problem**: Reference outlives data
```rust
// ❌ Wrong
fn get_first(v: &Vec<String>) -> &String {
    &v[0]
}  // OK, lifetime of return tied to input

fn make_and_get() -> &String {  // ❌ Missing lifetime parameter
    let v = vec!["hello".to_string()];
    &v[0]  // ❌ Error: v dropped, reference invalid
}
```

**Solution**: Return owned data or use lifetimes correctly
```rust
// ✅ Correct - return owned
fn make_and_get() -> String {
    let v = vec!["hello".to_string()];
    v[0].clone()
}

// ✅ Correct - explicit lifetime
fn get_first<'a>(v: &'a Vec<String>) -> &'a String {
    &v[0]
}
```

### 4. Async Function Calls

**Problem**: Forgetting `.await`
```rust
// ❌ Wrong
async fn fetch_data() -> Result<Data> {
    let data = load_from_disk();  // Returns Future, not Data!
    Ok(data)  // ❌ Type mismatch
}
```

**Solution**: Use `.await`
```rust
// ✅ Correct
async fn fetch_data() -> Result<Data> {
    let data = load_from_disk().await?;
    Ok(data)
}
```

### 5. Blocking in Async Context

**Problem**: Blocking the async runtime
```rust
// ❌ Wrong - blocks async runtime
async fn process() {
    let data = std::fs::read_to_string("file.txt").unwrap();  // Blocking!
    // Process data
}
```

**Solution**: Use async I/O
```rust
// ✅ Correct - async I/O
async fn process() {
    let data = tokio::fs::read_to_string("file.txt").await?;
    // Process data
}
```

---

## Migration Checklist

When migrating a C++ class:

- [ ] Convert class to struct
- [ ] Convert methods to impl block
- [ ] Replace pointers with owned types or references
- [ ] Replace signals/slots with channels
- [ ] Add proper error handling (Result)
- [ ] Make blocking I/O async
- [ ] Replace QThread with tokio tasks
- [ ] Replace Qt containers with std::collections
- [ ] Add proper lifetime annotations
- [ ] Write unit tests
- [ ] Document public API

## Example: Complete Class Migration

**C++ (Candle) - serialportconnection.h**:
```cpp
class SerialPortConnection : public Connection {
    Q_OBJECT
private:
    QSerialPort* m_port;
    QByteArray m_buffer;
    
public:
    SerialPortConnection(QObject* parent = nullptr);
    ~SerialPortConnection();
    
    void connect() override;
    void disconnect() override;
    bool isConnected() const override;
    void send(const QByteArray& data) override;
    
private slots:
    void onReadyRead();
    void onErrorOccurred(QSerialPort::SerialPortError error);
};
```

**Rust (rCandle) - serial.rs**:
```rust
use tokio::sync::mpsc;
use tokio_serial::{SerialPort, SerialStream};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Port not connected")]
    NotConnected,
}

pub struct SerialConnection {
    port: Option<SerialStream>,
    buffer: Vec<u8>,
    data_tx: mpsc::Sender<Vec<u8>>,
    error_tx: mpsc::Sender<SerialError>,
}

impl SerialConnection {
    pub fn new(
        data_tx: mpsc::Sender<Vec<u8>>,
        error_tx: mpsc::Sender<SerialError>,
    ) -> Self {
        Self {
            port: None,
            buffer: Vec::new(),
            data_tx,
            error_tx,
        }
    }
    
    pub async fn connect(&mut self, port_name: &str, baud_rate: u32) -> Result<(), SerialError> {
        let port = tokio_serial::new(port_name, baud_rate).open_native_async()?;
        self.port = Some(port);
        
        // Start read loop
        self.start_read_loop();
        
        Ok(())
    }
    
    pub async fn disconnect(&mut self) -> Result<(), SerialError> {
        self.port = None;
        Ok(())
    }
    
    pub fn is_connected(&self) -> bool {
        self.port.is_some()
    }
    
    pub async fn send(&mut self, data: &[u8]) -> Result<(), SerialError> {
        if let Some(port) = &mut self.port {
            use tokio::io::AsyncWriteExt;
            port.write_all(data).await?;
            Ok(())
        } else {
            Err(SerialError::NotConnected)
        }
    }
    
    fn start_read_loop(&mut self) {
        // Implementation of read loop
        // Similar to onReadyRead but in async loop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_connect() {
        // Test implementation
    }
}
```

---

This guide should help you translate patterns from C++/Qt Candle to Rust rCandle. Remember: when in doubt, consult the Rust Book and tokio documentation!
