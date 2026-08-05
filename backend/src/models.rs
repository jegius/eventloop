use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for tasks
pub type TaskId = String;

/// Represents a single frame/function in the call stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: String,
    pub function_name: String,
    pub file_name: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub source_code: Option<String>,
    pub execution_context: ExecutionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionContext {
    Global,
    Function(String),
    Module(String),
    Eval,
    Constructor,
    Async,
    Generator,
}

impl Default for StackFrame {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            function_name: "anonymous".to_string(),
            file_name: None,
            line_number: None,
            column_number: None,
            source_code: None,
            execution_context: ExecutionContext::Global,
        }
    }
}

/// Microtask (Promise callbacks, queueMicrotask, MutationObserver)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Microtask {
    pub id: TaskId,
    pub task_type: MicrotaskType,
    pub callback: String,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub priority: u8,
    pub parent_task_id: Option<TaskId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MicrotaskType {
    PromiseThen,
    PromiseCatch,
    PromiseFinally,
    QueueMicrotask,
    MutationObserver,
    Custom,
}

/// Macrotask (setTimeout, setInterval, I/O, UI events)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macrotask {
    pub id: TaskId,
    pub task_type: MacrotaskType,
    pub callback: String,
    pub delay_ms: Option<u64>,
    pub scheduled_time: DateTime<Utc>,
    pub source: String,
    pub repeat: bool,
    pub interval_id: Option<TaskId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MacrotaskType {
    SetTimeout,
    SetInterval,
    UIEvent(String), // click, keydown, etc.
    NetworkEvent,
    PostMessage,
    Custom,
}

/// Render Queue task (for visualizing render phases)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTask {
    pub id: TaskId,
    pub task_type: RenderTaskType,
    pub description: String,
    pub phase: RenderPhase,
    pub duration_estimate_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RenderTaskType {
    ParsingHTML,
    ParsingCSS,
    DOMConstruction,
    CSSOMConstruction,
    RenderTree,
    Layout,
    Paint,
    Composite,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RenderPhase {
    Parsing,
    Style,
    Layout,
    Paint,
    Composite,
    Idle,
}

/// RequestAnimationFrame task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAFTask {
    pub id: TaskId,
    pub callback: String,
    pub scheduled_frame: u64,
    pub source: String,
    pub created_at: DateTime<Utc>,
}

/// RequestIdleCallback task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RICTask {
    pub id: TaskId,
    pub callback: String,
    pub timeout_ms: Option<u64>,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub deadline_remaining_ms: Option<u64>,
}

/// Checkpoint for performance tasks and user interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub checkpoint_type: CheckpointType,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub associated_task_id: Option<TaskId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckpointType {
    PerformanceMark,
    PerformanceMeasure,
    UserInteractionStart,
    UserInteractionEnd,
    GCStart,
    GCEnd,
    ScriptStart,
    ScriptEnd,
}

/// Complete state of the Event Loop at a given moment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLoopState {
    pub timestamp: DateTime<Utc>,
    pub step_number: u64,
    pub phase: EventLoopPhase,
    
    // Call Stack (LIFO)
    pub call_stack: Vec<StackFrame>,
    
    // Queues
    pub microtask_queue: Vec<Microtask>,
    pub macrotask_queue: Vec<Macrotask>,
    pub render_queue: Vec<RenderTask>,
    pub raf_queue: Vec<RAFTask>,
    pub ric_queue: Vec<RICTask>,
    
    // Checkpoints and metrics
    pub checkpoints: Vec<Checkpoint>,
    pub current_render_phase: Option<RenderPhase>,
    
    // Execution metadata
    pub is_running: bool,
    pub is_paused: bool,
    pub breakpoint_hit: Option<BreakpointInfo>,
    pub execution_speed_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventLoopPhase {
    Initializing,
    ExecutingScript,
    ProcessingMicrotasks,
    ProcessingMacrotasks,
    Rendering,
    Idle,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointInfo {
    pub file_name: String,
    pub line_number: u32,
    pub condition: Option<String>,
    pub hit_count: u32,
}

impl Default for EventLoopState {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            step_number: 0,
            phase: EventLoopPhase::Initializing,
            call_stack: Vec::new(),
            microtask_queue: Vec::new(),
            macrotask_queue: Vec::new(),
            render_queue: Vec::new(),
            raf_queue: Vec::new(),
            ric_queue: Vec::new(),
            checkpoints: Vec::new(),
            current_render_phase: None,
            is_running: false,
            is_paused: false,
            breakpoint_hit: None,
            execution_speed_ms: 100,
        }
    }
}

/// Console output message (V8-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleMessage {
    pub id: String,
    pub level: ConsoleLevel,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub stack_trace: Option<Vec<StackFrame>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsoleLevel {
    Log,
    Info,
    Warn,
    Error,
    Debug,
    Trace,
    Table,
    Dir,
    Group,
    GroupCollapsed,
    GroupEnd,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    // Client -> Server
    ExecuteCode { code: String },
    StepOver,
    StepInto,
    StepOut,
    Play,
    Pause,
    Stop,
    Rewind { steps: u64 },
    SetBreakpoint { file: String, line: u32, condition: Option<String> },
    RemoveBreakpoint { file: String, line: u32 },
    SetSpeed { speed_ms: u32 },
    
    // Server -> Client
    StateUpdate(EventLoopState),
    ConsoleOutput(ConsoleMessage),
    ExecutionComplete { result: serde_json::Value },
    Error { message: String, code: Option<String> },
    BreakpointHit(BreakpointInfo),
    HistoryUpdate { available_steps: u64, current_step: u64 },
}
