import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { EventLoopState, ConsoleMessage, RenderPhase } from '../types.js';

/**
 * Main Application Component
 */
@customElement('el-app')
export class App extends LitElement {
  @property({ type: Object })
  currentState: EventLoopState | null = null;

  @property({ type: Array })
  consoleMessages: ConsoleMessage[] = [];

  static override styles = css`
    :host {
      display: block;
    }
  `;

  private client: any = null;
  private speedValue = 100;

  override firstUpdated() {
    this.initializeWebSocket();
    this.setupEventListeners();
  }

  private async initializeWebSocket() {
    // Dynamic import to avoid circular dependencies
    const { client } = await import('./core/client.js');
    this.client = client;

    this.client.onConnect(() => {
      this.updateStatus('Connected', true);
    });

    this.client.onDisconnect(() => {
      this.updateStatus('Disconnected', false);
    });

    this.client.onError((error: string) => {
      console.error('Client error:', error);
      this.addConsoleMessage({
        id: Date.now().toString(),
        level: 'Error',
        message: error,
        data: null,
        timestamp: new Date().toISOString(),
        source: 'system',
        stack_trace: null,
      });
    });

    this.client.onStateUpdate((state: EventLoopState) => {
      this.currentState = state;
      this.requestUpdate();
    });

    // Connect to server
    this.client.connect().catch(console.error);
  }

  private setupEventListeners() {
    // Run code button
    document.getElementById('runCodeBtn')?.addEventListener('click', () => {
      const codeEditor = document.getElementById('codeEditor') as HTMLTextAreaElement;
      if (codeEditor && this.client) {
        this.client.executeCode(codeEditor.value);
      }
    });

    // Clear code button
    document.getElementById('clearCodeBtn')?.addEventListener('click', () => {
      const codeEditor = document.getElementById('codeEditor') as HTMLTextAreaElement;
      if (codeEditor) {
        codeEditor.value = '';
      }
    });

    // Control buttons
    document.getElementById('playBtn')?.addEventListener('click', () => {
      this.client?.play();
    });

    document.getElementById('pauseBtn')?.addEventListener('click', () => {
      this.client?.pause();
    });

    document.getElementById('stopBtn')?.addEventListener('click', () => {
      this.client?.stop();
    });

    document.getElementById('stepOverBtn')?.addEventListener('click', () => {
      this.client?.stepOver();
    });

    document.getElementById('stepIntoBtn')?.addEventListener('click', () => {
      this.client?.stepInto();
    });

    document.getElementById('stepOutBtn')?.addEventListener('click', () => {
      this.client?.stepOut();
    });

    document.getElementById('rewindBtn')?.addEventListener('click', () => {
      this.client?.rewind(1);
    });

    // Speed slider
    const speedSlider = document.getElementById('speedSlider') as HTMLInputElement;
    const speedValue = document.getElementById('speedValue');
    
    speedSlider?.addEventListener('input', (e) => {
      const value = (e.target as HTMLInputElement).value;
      this.speedValue = parseInt(value);
      if (speedValue) {
        speedValue.textContent = `${this.speedValue}ms`;
      }
      this.client?.setSpeed(this.speedValue);
    });

    // Clear console button
    document.getElementById('clearConsoleBtn')?.addEventListener('click', () => {
      this.consoleMessages = [];
      this.requestUpdate();
    });
  }

  private updateStatus(text: string, connected: boolean) {
    const statusText = document.getElementById('statusText');
    const statusDot = document.getElementById('statusDot');
    
    if (statusText) {
      statusText.textContent = text;
    }
    
    if (statusDot) {
      statusDot.className = `status-dot ${connected ? 'connected' : 'disconnected'}`;
    }
  }

  private addConsoleMessage(message: ConsoleMessage) {
    this.consoleMessages = [...this.consoleMessages, message];
    this.requestUpdate();
  }

  override render() {
    // Update child components with current state
    this.updateChildComponents();
    
    return html`<slot></slot>`;
  }

  private updateChildComponents() {
    if (!this.currentState) return;

    // Update call stack
    const callStack = document.getElementById('callStack') as any;
    if (callStack) {
      callStack.frames = this.currentState.call_stack;
    }

    // Update queues
    const microtaskQueue = document.getElementById('microtaskQueue') as any;
    const macrotaskQueue = document.getElementById('macrotaskQueue') as any;
    const renderQueue = document.getElementById('renderQueue') as any;
    const rafQueue = document.getElementById('rafQueue') as any;
    const ricQueue = document.getElementById('ricQueue') as any;

    if (microtaskQueue) microtaskQueue.microtasks = this.currentState.microtask_queue;
    if (macrotaskQueue) macrotaskQueue.macrotasks = this.currentState.macrotask_queue;
    if (renderQueue) renderQueue.renderTasks = this.currentState.render_queue;
    if (rafQueue) rafQueue.rafTasks = this.currentState.raf_queue;
    if (ricQueue) ricQueue.ricTasks = this.currentState.ric_queue;

    // Update render phases
    const renderPhases = document.getElementById('renderPhases') as any;
    if (renderPhases) {
      renderPhases.currentPhase = this.currentState.current_render_phase;
    }

    // Update console
    const consoleOutput = document.getElementById('consoleOutput') as any;
    if (consoleOutput) {
      consoleOutput.messages = this.consoleMessages;
    }
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'el-app': App;
  }
}
