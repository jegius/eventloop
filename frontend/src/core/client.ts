import type { EventLoopState, ConsoleMessage, ClientMessage, ServerMessage } from './types.js';

/**
 * WebSocket client for communicating with the Rust backend
 */
export class EventLoopClient {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  
  private stateListeners: ((state: EventLoopState) => void)[] = [];
  private consoleListeners: ((message: ConsoleMessage) => void)[] = [];
  private errorListeners: ((error: string) => void)[] = [];
  private disconnectListeners: (() => void)[] = [];
  private connectListeners: (() => void)[] = [];

  constructor(private url: string = 'ws://localhost:8080') {}

  /**
   * Connect to the WebSocket server
   */
  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.url);

        this.ws.onopen = () => {
          console.log('WebSocket connected');
          this.reconnectAttempts = 0;
          this.connectListeners.forEach(listener => listener());
          resolve();
        };

        this.ws.onmessage = (event) => {
          this.handleMessage(event.data);
        };

        this.ws.onerror = (error) => {
          console.error('WebSocket error:', error);
          this.errorListeners.forEach(listener => listener('Connection error'));
          reject(error);
        };

        this.ws.onclose = () => {
          console.log('WebSocket closed');
          this.disconnectListeners.forEach(listener => listener());
          this.attemptReconnect();
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Disconnect from the server
   */
  disconnect(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  /**
   * Send a message to the server
   */
  send(message: ClientMessage): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('WebSocket is not connected');
    }
    
    const json = JSON.stringify(message);
    this.ws.send(json);
  }

  /**
   * Execute JavaScript code
   */
  executeCode(code: string): void {
    this.send({ type: 'ExecuteCode', payload: { code } });
  }

  /**
   * Control execution
   */
  stepOver(): void {
    this.send({ type: 'StepOver', payload: null });
  }

  stepInto(): void {
    this.send({ type: 'StepInto', payload: null });
  }

  stepOut(): void {
    this.send({ type: 'StepOut', payload: null });
  }

  play(): void {
    this.send({ type: 'Play', payload: null });
  }

  pause(): void {
    this.send({ type: 'Pause', payload: null });
  }

  stop(): void {
    this.send({ type: 'Stop', payload: null });
  }

  rewind(steps: number): void {
    this.send({ type: 'Rewind', payload: { steps } });
  }

  setBreakpoint(file: string, line: number, condition?: string): void {
    this.send({ 
      type: 'SetBreakpoint', 
      payload: { file, line, condition: condition ?? null } 
    });
  }

  removeBreakpoint(file: string, line: number): void {
    this.send({ type: 'RemoveBreakpoint', payload: { file, line } });
  }

  setSpeed(speedMs: number): void {
    this.send({ type: 'SetSpeed', payload: { speed_ms: speedMs } });
  }

  /**
   * Subscribe to state updates
   */
  onStateUpdate(listener: (state: EventLoopState) => void): () => void {
    this.stateListeners.push(listener);
    return () => {
      const index = this.stateListeners.indexOf(listener);
      if (index > -1) {
        this.stateListeners.splice(index, 1);
      }
    };
  }

  /**
   * Subscribe to console messages
   */
  onConsoleMessage(listener: (message: ConsoleMessage) => void): () => void {
    this.consoleListeners.push(listener);
    return () => {
      const index = this.consoleListeners.indexOf(listener);
      if (index > -1) {
        this.consoleListeners.splice(index, 1);
      }
    };
  }

  /**
   * Subscribe to errors
   */
  onError(listener: (error: string) => void): () => void {
    this.errorListeners.push(listener);
    return () => {
      const index = this.errorListeners.indexOf(listener);
      if (index > -1) {
        this.errorListeners.splice(index, 1);
      }
    };
  }

  /**
   * Subscribe to disconnect events
   */
  onDisconnect(listener: () => void): () => void {
    this.disconnectListeners.push(listener);
    return () => {
      const index = this.disconnectListeners.indexOf(listener);
      if (index > -1) {
        this.disconnectListeners.splice(index, 1);
      }
    };
  }

  /**
   * Subscribe to connect events
   */
  onConnect(listener: () => void): () => void {
    this.connectListeners.push(listener);
    return () => {
      const index = this.connectListeners.indexOf(listener);
      if (index > -1) {
        this.connectListeners.splice(index, 1);
      }
    };
  }

  /**
   * Check if connected
   */
  isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  /**
   * Handle incoming messages
   */
  private handleMessage(data: string): void {
    try {
      const message: ServerMessage = JSON.parse(data);

      switch (message.type) {
        case 'StateUpdate':
          this.stateListeners.forEach(listener => listener(message.payload));
          break;

        case 'ConsoleOutput':
          this.consoleListeners.forEach(listener => listener(message.payload));
          break;

        case 'Error':
          this.errorListeners.forEach(listener => listener(message.payload.message));
          break;

        case 'BreakpointHit':
          console.log('Breakpoint hit:', message.payload);
          break;

        case 'HistoryUpdate':
          console.log('History updated:', message.payload);
          break;

        case 'ExecutionComplete':
          console.log('Execution complete:', message.payload);
          break;
      }
    } catch (error) {
      console.error('Error parsing message:', error);
    }
  }

  /**
   * Attempt to reconnect
   */
  private attemptReconnect(): void {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`Reconnecting... (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      
      setTimeout(() => {
        this.connect().catch(console.error);
      }, this.reconnectDelay * this.reconnectAttempts);
    } else {
      console.error('Max reconnect attempts reached');
      this.errorListeners.forEach(listener => listener('Connection lost'));
    }
  }
}

// Export singleton instance
export const client = new EventLoopClient();
