/**
 * Main entry point for the JS Event Loop Visualizer
 */

// Import Web Components
import './components/app.js';
import './components/call-stack.js';
import './components/task-queue.js';
import './components/render-phases.js';
import './components/console-output.js';

// Initialize application when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  console.log('🚀 JS Event Loop Visualizer initialized');
  
  // Create app element if not exists
  const app = document.querySelector('el-app');
  if (!app) {
    const newApp = document.createElement('el-app');
    document.body.appendChild(newApp);
  }
});

// Export types for external use
export type { 
  EventLoopState, 
  ConsoleMessage, 
  StackFrame,
  Microtask,
  Macrotask,
  RenderPhase 
} from './types.js';

export { client } from './core/client.js';
