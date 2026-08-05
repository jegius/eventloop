/**
 * TypeScript types for JS Event Loop Visualizer
 * Mirrors the Rust backend models
 */

export type TaskId = string;

export interface StackFrame {
  id: string;
  function_name: string;
  file_name: string | null;
  line_number: number | null;
  column_number: number | null;
  source_code: string | null;
  execution_context: ExecutionContext;
}

export type ExecutionContext = 
  | 'Global'
  | `Function:${string}`
  | `Module:${string}`
  | 'Eval'
  | 'Constructor'
  | 'Async'
  | 'Generator';

export interface Microtask {
  id: TaskId;
  task_type: MicrotaskType;
  callback: string;
  source: string;
  created_at: string;
  priority: number;
  parent_task_id: TaskId | null;
}

export type MicrotaskType = 
  | 'PromiseThen'
  | 'PromiseCatch'
  | 'PromiseFinally'
  | 'QueueMicrotask'
  | 'MutationObserver'
  | 'Custom';

export interface Macrotask {
  id: TaskId;
  task_type: MacrotaskType;
  callback: string;
  delay_ms: number | null;
  scheduled_time: string;
  source: string;
  repeat: boolean;
  interval_id: TaskId | null;
}

export type MacrotaskType = 
  | 'SetTimeout'
  | 'SetInterval'
  | `UIEvent:${string}`
  | 'NetworkEvent'
  | 'PostMessage'
  | 'Custom';

export interface RenderTask {
  id: TaskId;
  task_type: RenderTaskType;
  description: string;
  phase: RenderPhase;
  duration_estimate_ms: number;
}

export type RenderTaskType = 
  | 'ParsingHTML'
  | 'ParsingCSS'
  | 'DOMConstruction'
  | 'CSSOMConstruction'
  | 'RenderTree'
  | 'Layout'
  | 'Paint'
  | 'Composite';

export type RenderPhase = 
  | 'Parsing'
  | 'Style'
  | 'Layout'
  | 'Paint'
  | 'Composite'
  | 'Idle';

export interface RAFTask {
  id: TaskId;
  callback: string;
  scheduled_frame: number;
  source: string;
  created_at: string;
}

export interface RICTask {
  id: TaskId;
  callback: string;
  timeout_ms: number | null;
  source: string;
  created_at: string;
  deadline_remaining_ms: number | null;
}

export interface Checkpoint {
  id: string;
  checkpoint_type: CheckpointType;
  timestamp: string;
  description: string;
  associated_task_id: TaskId | null;
}

export type CheckpointType = 
  | 'PerformanceMark'
  | 'PerformanceMeasure'
  | 'UserInteractionStart'
  | 'UserInteractionEnd'
  | 'GCStart'
  | 'GCEnd'
  | 'ScriptStart'
  | 'ScriptEnd';

export type EventLoopPhase = 
  | 'Initializing'
  | 'ExecutingScript'
  | 'ProcessingMicrotasks'
  | 'ProcessingMacrotasks'
  | 'Rendering'
  | 'Idle'
  | 'Stopped';

export interface BreakpointInfo {
  file_name: string;
  line_number: number;
  condition: string | null;
  hit_count: number;
}

export interface EventLoopState {
  timestamp: string;
  step_number: number;
  phase: EventLoopPhase;
  call_stack: StackFrame[];
  microtask_queue: Microtask[];
  macrotask_queue: Macrotask[];
  render_queue: RenderTask[];
  raf_queue: RAFTask[];
  ric_queue: RICTask[];
  checkpoints: Checkpoint[];
  current_render_phase: RenderPhase | null;
  is_running: boolean;
  is_paused: boolean;
  breakpoint_hit: BreakpointInfo | null;
  execution_speed_ms: number;
}

export type ConsoleLevel = 
  | 'Log'
  | 'Info'
  | 'Warn'
  | 'Error'
  | 'Debug'
  | 'Trace'
  | 'Table'
  | 'Dir'
  | 'Group'
  | 'GroupCollapsed'
  | 'GroupEnd';

export interface ConsoleMessage {
  id: string;
  level: ConsoleLevel;
  message: string;
  data: unknown | null;
  timestamp: string;
  source: string;
  stack_trace: StackFrame[] | null;
}

// WebSocket Message Types
export type ClientMessage = 
  | { type: 'ExecuteCode'; payload: { code: string } }
  | { type: 'StepOver'; payload: null }
  | { type: 'StepInto'; payload: null }
  | { type: 'StepOut'; payload: null }
  | { type: 'Play'; payload: null }
  | { type: 'Pause'; payload: null }
  | { type: 'Stop'; payload: null }
  | { type: 'Rewind'; payload: { steps: number } }
  | { type: 'SetBreakpoint'; payload: { file: string; line: number; condition: string | null } }
  | { type: 'RemoveBreakpoint'; payload: { file: string; line: number } }
  | { type: 'SetSpeed'; payload: { speed_ms: number } };

export type ServerMessage = 
  | { type: 'StateUpdate'; payload: EventLoopState }
  | { type: 'ConsoleOutput'; payload: ConsoleMessage }
  | { type: 'ExecutionComplete'; payload: { result: unknown } }
  | { type: 'Error'; payload: { message: string; code: string | null } }
  | { type: 'BreakpointHit'; payload: BreakpointInfo }
  | { type: 'HistoryUpdate'; payload: { available_steps: number; current_step: number } };
