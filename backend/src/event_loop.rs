use crate::models::*;
use chrono::Utc;
use std::collections::VecDeque;
use uuid::Uuid;

/// Event Loop Engine implementing ECMAScript 2026 specification
pub struct EventLoopEngine {
    state: EventLoopState,
    history: Vec<EventLoopState>,
    history_index: usize,
    max_history_size: usize,
    breakpoints: Vec<(String, u32, Option<String>)>, // (file, line, condition)
}

impl EventLoopEngine {
    pub fn new() -> Self {
        Self {
            state: EventLoopState::default(),
            history: Vec::with_capacity(1000),
            history_index: 0,
            max_history_size: 1000,
            breakpoints: Vec::new(),
        }
    }

    /// Execute JavaScript code and update event loop state
    pub fn execute_code(&mut self, code: &str) -> Result<(), String> {
        self.save_state();
        
        self.state.phase = EventLoopPhase::ExecutingScript;
        self.state.is_running = true;
        self.state.is_paused = false;
        
        // Add script start checkpoint
        self.add_checkpoint(CheckpointType::ScriptStart, "Script execution started".to_string(), None);
        
        // Parse and analyze the code to extract tasks
        self.parse_and_schedule_tasks(code)?;
        
        // Process one step of the event loop
        self.process_event_loop_step()?;
        
        self.broadcast_state();
        Ok(())
    }

    /// Step over - execute current statement and move to next
    pub fn step_over(&mut self) -> Result<(), String> {
        if !self.state.is_running {
            return Err("No active execution".to_string());
        }
        
        self.save_state();
        self.process_event_loop_step()?;
        self.broadcast_state();
        Ok(())
    }

    /// Step into - dive into function calls
    pub fn step_into(&mut self) -> Result<(), String> {
        if !self.state.is_running {
            return Err("No active execution".to_string());
        }
        
        self.save_state();
        // Similar to step_over but will enter function frames
        self.process_event_loop_step()?;
        self.broadcast_state();
        Ok(())
    }

    /// Step out - complete current function and return
    pub fn step_out(&mut self) -> Result<(), String> {
        if !self.state.is_running {
            return Err("No active execution".to_string());
        }
        
        self.save_state();
        // Pop current frame and continue until function completes
        if let Some(_frame) = self.state.call_stack.pop() {
            self.process_event_loop_step()?;
        }
        self.broadcast_state();
        Ok(())
    }

    /// Play - continue execution until completion or breakpoint
    pub fn play(&mut self) -> Result<(), String> {
        if !self.state.is_running && self.state.step_number == 0 {
            return Err("No code loaded. Execute code first.".to_string());
        }
        
        self.state.is_running = true;
        self.state.is_paused = false;
        
        // Continue processing until done or breakpoint
        while self.state.is_running && !self.state.is_paused {
            match self.process_event_loop_step() {
                Ok(_) => {
                    self.broadcast_state();
                    // Check for breakpoints
                    if self.check_breakpoints() {
                        self.state.is_paused = true;
                        break;
                    }
                }
                Err(e) => {
                    self.state.is_running = false;
                    return Err(e);
                }
            }
        }
        
        Ok(())
    }

    /// Pause execution
    pub fn pause(&mut self) -> Result<(), String> {
        self.state.is_paused = true;
        self.broadcast_state();
        Ok(())
    }

    /// Stop execution
    pub fn stop(&mut self) -> Result<(), String> {
        self.state.is_running = false;
        self.state.is_paused = false;
        self.state.phase = EventLoopPhase::Stopped;
        self.state.call_stack.clear();
        self.broadcast_state();
        Ok(())
    }

    /// Rewind to a previous state
    pub fn rewind(&mut self, steps: u64) -> Result<(), String> {
        if steps == 0 || self.history.is_empty() {
            return Ok(());
        }
        
        let target_index = self.history_index.saturating_sub(steps as usize);
        if target_index < self.history.len() {
            self.history_index = target_index;
            self.state = self.history[target_index].clone();
            self.broadcast_state();
        }
        
        Ok(())
    }

    /// Set a breakpoint
    pub fn set_breakpoint(&mut self, file: String, line: u32, condition: Option<String>) {
        self.breakpoints.push((file, line, condition));
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, file: String, line: u32) {
        self.breakpoints.retain(|(f, l, _)| f != &file || l != &line);
    }

    /// Set execution speed
    pub fn set_speed(&mut self, speed_ms: u32) {
        self.state.execution_speed_ms = speed_ms;
    }

    // Private methods

    fn save_state(&mut self) {
        if self.history.len() >= self.max_history_size {
            self.history.remove(0);
            if self.history_index > 0 {
                self.history_index -= 1;
            }
        }
        
        let state_copy = self.state.clone();
        self.history.push(state_copy);
        self.history_index = self.history.len() - 1;
    }

    fn broadcast_state(&self) {
        // In real implementation, this would send via WebSocket
        // For now, we just log
        log::debug!("State updated: step={}, phase={:?}", 
            self.state.step_number, self.state.phase);
    }

    fn check_breakpoints(&self) -> bool {
        if let Some(frame) = self.state.call_stack.last() {
            if let (Some(file), Some(line)) = (&frame.file_name, frame.line_number) {
                for (bp_file, bp_line, condition) in &self.breakpoints {
                    if bp_file == file && *bp_line == line {
                        if let Some(cond) = condition {
                            // Evaluate condition (simplified)
                            if cond == "true" {
                                return true;
                            }
                        } else {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn add_checkpoint(&mut self, checkpoint_type: CheckpointType, description: String, task_id: Option<TaskId>) {
        let checkpoint = Checkpoint {
            id: Uuid::new_v4().to_string(),
            checkpoint_type,
            timestamp: Utc::now(),
            description,
            associated_task_id: task_id,
        };
        self.state.checkpoints.push(checkpoint);
    }

    fn parse_and_schedule_tasks(&mut self, code: &str) -> Result<(), String> {
        // Simplified parser for demonstration
        // In production, this would use a real JS parser
        
        let lines: Vec<&str> = code.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            // Detect console.log
            if line.starts_with("console.log") {
                let msg = extract_console_message(line);
                self.send_console_output(ConsoleLevel::Log, msg, None);
            }
            
            // Detect setTimeout
            if line.contains("setTimeout") {
                let delay = extract_timeout_delay(line);
                let task = Macrotask {
                    id: Uuid::new_v4().to_string(),
                    task_type: MacrotaskType::SetTimeout,
                    callback: line.to_string(),
                    delay_ms: Some(delay),
                    scheduled_time: Utc::now(),
                    source: "user_code".to_string(),
                    repeat: false,
                    interval_id: None,
                };
                self.state.macrotask_queue.push(task);
            }
            
            // Detect Promise
            if line.contains("new Promise") || line.contains("Promise.resolve") || line.contains("Promise.reject") {
                let task = Microtask {
                    id: Uuid::new_v4().to_string(),
                    task_type: MicrotaskType::PromiseThen,
                    callback: line.to_string(),
                    source: "user_code".to_string(),
                    created_at: Utc::now(),
                    priority: 1,
                    parent_task_id: None,
                };
                self.state.microtask_queue.push(task);
            }
            
            // Detect queueMicrotask
            if line.contains("queueMicrotask") {
                let task = Microtask {
                    id: Uuid::new_v4().to_string(),
                    task_type: MicrotaskType::QueueMicrotask,
                    callback: line.to_string(),
                    source: "user_code".to_string(),
                    created_at: Utc::now(),
                    priority: 0,
                    parent_task_id: None,
                };
                self.state.microtask_queue.push(task);
            }
            
            // Detect requestAnimationFrame
            if line.contains("requestAnimationFrame") {
                let task = RAFTask {
                    id: Uuid::new_v4().to_string(),
                    callback: line.to_string(),
                    scheduled_frame: 1,
                    source: "user_code".to_string(),
                    created_at: Utc::now(),
                };
                self.state.raf_queue.push(task);
            }
            
            // Add stack frame for top-level code
            if line_num == 0 {
                let frame = StackFrame {
                    function_name: "<global>".to_string(),
                    file_name: Some("script.js".to_string()),
                    line_number: Some((line_num + 1) as u32),
                    column_number: Some(1),
                    source_code: Some(line.to_string()),
                    execution_context: ExecutionContext::Global,
                    ..Default::default()
                };
                self.state.call_stack.push(frame);
            }
        }
        
        Ok(())
    }

    fn process_event_loop_step(&mut self) -> Result<(), String> {
        self.state.step_number += 1;
        
        // ECMAScript 2026 Event Loop Algorithm:
        // 1. Execute oldest microtask (if any)
        // 2. If microtask queue empty, execute oldest macrotask
        // 3. Process render queue after macrotask
        // 4. Process RAF callbacks before paint
        // 5. Process RIC callbacks during idle periods
        
        // Priority 1: Process all microtasks
        if !self.state.microtask_queue.is_empty() {
            self.state.phase = EventLoopPhase::ProcessingMicrotasks;
            
            if let Some(microtask) = self.state.microtask_queue.pop_front_like() {
                self.execute_microtask(microtask)?;
                return Ok(());
            }
        }
        
        // Priority 2: Process macrotasks
        if !self.state.macrotask_queue.is_empty() {
            self.state.phase = EventLoopPhase::ProcessingMacrotasks;
            
            // Sort by scheduled time
            self.state.macrotask_queue.sort_by(|a, b| {
                a.scheduled_time.cmp(&b.scheduled_time)
            });
            
            if let Some(macrotask) = self.state.macrotask_queue.first() {
                let task = macrotask.clone();
                self.execute_macrotask(task)?;
                return Ok(());
            }
        }
        
        // Priority 3: Process render queue
        if !self.state.render_queue.is_empty() {
            self.state.phase = EventLoopPhase::Rendering;
            
            if let Some(render_task) = self.state.render_queue.pop_front_like() {
                self.execute_render_task(render_task)?;
                return Ok(());
            }
        }
        
        // Priority 4: Process RAF callbacks
        if !self.state.raf_queue.is_empty() {
            if let Some(raf_task) = self.state.raf_queue.pop_front_like() {
                self.execute_raf_task(raf_task)?;
                return Ok(());
            }
        }
        
        // Priority 5: Process RIC callbacks
        if !self.state.ric_queue.is_empty() {
            if let Some(ric_task) = self.state.ric_queue.pop_front_like() {
                self.execute_ric_task(ric_task)?;
                return Ok(());
            }
        }
        
        // No more tasks - execution complete
        self.state.is_running = false;
        self.state.phase = EventLoopPhase::Idle;
        self.add_checkpoint(CheckpointType::ScriptEnd, "All tasks completed".to_string(), None);
        
        Ok(())
    }

    fn execute_microtask(&mut self, task: Microtask) -> Result<(), String> {
        // Push to call stack
        let frame = StackFrame {
            function_name: format!("{:?}", task.task_type),
            source_code: Some(task.callback.clone()),
            execution_context: ExecutionContext::Async,
            ..Default::default()
        };
        self.state.call_stack.push(frame);
        
        // Simulate execution
        self.send_console_output(
            ConsoleLevel::Debug,
            format!("Executing microtask: {:?}", task.task_type),
            None,
        );
        
        // Pop from stack
        self.state.call_stack.pop();
        
        Ok(())
    }

    fn execute_macrotask(&mut self, mut task: Macrotask) -> Result<(), String> {
        // Push to call stack
        let frame = StackFrame {
            function_name: format!("{:?}", task.task_type),
            source_code: Some(task.callback.clone()),
            execution_context: ExecutionContext::Function("macrotask".to_string()),
            ..Default::default()
        };
        self.state.call_stack.push(frame);
        
        self.send_console_output(
            ConsoleLevel::Debug,
            format!("Executing macrotask: {:?} (delay: {:?}ms)", task.task_type, task.delay_ms),
            None,
        );
        
        // Handle setInterval repeat
        if task.repeat {
            task.scheduled_time = Utc::now();
            self.state.macrotask_queue.push(task);
        } else {
            // Remove from queue (already removed by pop_front_like simulation)
            self.state.macrotask_queue.remove(0);
        }
        
        // Pop from stack
        self.state.call_stack.pop();
        
        Ok(())
    }

    fn execute_render_task(&mut self, task: RenderTask) -> Result<(), String> {
        self.state.current_render_phase = Some(task.phase.clone());
        
        self.send_console_output(
            ConsoleLevel::Info,
            format!("Render phase: {:?} - {}", task.phase, task.description),
            None,
        );
        
        self.state.current_render_phase = None;
        Ok(())
    }

    fn execute_raf_task(&mut self, task: RAFTask) -> Result<(), String> {
        let frame = StackFrame {
            function_name: "requestAnimationFrame".to_string(),
            source_code: Some(task.callback),
            execution_context: ExecutionContext::Async,
            ..Default::default()
        };
        self.state.call_stack.push(frame);
        
        self.send_console_output(
            ConsoleLevel::Debug,
            format!("Executing RAF callback (frame {})", task.scheduled_frame),
            None,
        );
        
        self.state.call_stack.pop();
        Ok(())
    }

    fn execute_ric_task(&mut self, task: RICTask) -> Result<(), String> {
        let frame = StackFrame {
            function_name: "requestIdleCallback".to_string(),
            source_code: Some(task.callback),
            execution_context: ExecutionContext::Async,
            ..Default::default()
        };
        self.state.call_stack.push(frame);
        
        self.send_console_output(
            ConsoleLevel::Debug,
            format!("Executing RIC callback (timeout: {:?}ms)", task.timeout_ms),
            None,
        );
        
        self.state.call_stack.pop();
        Ok(())
    }

    fn send_console_output(&mut self, level: ConsoleLevel, message: String, data: Option<serde_json::Value>) {
        let console_msg = ConsoleMessage {
            id: Uuid::new_v4().to_string(),
            level,
            message,
            data,
            timestamp: Utc::now(),
            source: "event_loop".to_string(),
            stack_trace: self.state.call_stack.clone().into(),
        };
        
        log::info!("Console: {:?} - {}", level, message);
        // In real implementation, send via WebSocket
    }
}

// Helper functions for parsing

fn extract_console_message(line: &str) -> String {
    // Simple extraction: console.log("message") -> "message"
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start+1..].find('"') {
            return line[start+1..start+1+end].to_string();
        }
    }
    if let Some(start) = line.find('\'') {
        if let Some(end) = line[start+1..].find('\'') {
            return line[start+1..start+1+end].to_string();
        }
    }
    "console output".to_string()
}

fn extract_timeout_delay(line: &str) -> u64 {
    // Simple extraction: setTimeout(..., 1000) -> 1000
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() >= 2 {
        if let Ok(delay) = parts[1].trim().trim_end_matches(')').parse::<u64>() {
            return delay;
        }
    }
    0
}

// Extension trait for VecDeque-like behavior on Vec
trait VecExt<T> {
    fn pop_front_like(&mut self) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn pop_front_like(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}
