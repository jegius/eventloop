import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { ConsoleMessage, ConsoleLevel } from '../types.js';

/**
 * Web Component for Console Output (V8-style)
 */
@customElement('el-console-output')
export class ConsoleOutputComponent extends LitElement {
  @property({ type: Array })
  messages: ConsoleMessage[] = [];

  private consoleContainer: HTMLElement | null = null;

  static override styles = css`
    :host {
      display: block;
      font-family: 'JetBrains Mono', 'Fira Code', monospace;
    }

    .console-container {
      height: 300px;
      background: #1e1e1e;
      border-radius: 8px;
      border: 1px solid #3a3a3a;
      overflow-y: auto;
      padding: 8px;
      scroll-behavior: smooth;
    }

    .console-message {
      display: flex;
      gap: 8px;
      padding: 6px 8px;
      border-bottom: 1px solid #2a2a2a;
      font-size: 13px;
      line-height: 1.5;
      animation: fadeIn 0.2s ease-out;
    }

    .console-message:last-child {
      border-bottom: none;
    }

    .message-level {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 20px;
      height: 20px;
      border-radius: 3px;
      font-size: 11px;
      font-weight: bold;
      flex-shrink: 0;
    }

    .message-level.log {
      background: #374151;
      color: #9ca3af;
    }

    .message-level.info {
      background: #1e40af;
      color: #60a5fa;
    }

    .message-level.warn {
      background: #78350f;
      color: #fbbf24;
    }

    .message-level.error {
      background: #7f1d1d;
      color: #f87171;
    }

    .message-level.debug {
      background: #312e81;
      color: #a5b4fc;
    }

    .message-level.trace {
      background: #115e59;
      color: #5eead4;
    }

    .message-content {
      flex: 1;
      color: #e5e7eb;
      word-break: break-word;
    }

    .message-timestamp {
      font-size: 10px;
      color: #6b7280;
      flex-shrink: 0;
      margin-left: 8px;
    }

    .message-source {
      font-size: 10px;
      color: #9ca3af;
      margin-top: 2px;
    }

    .message-data {
      margin-top: 4px;
      padding: 4px 8px;
      background: #2d2d2d;
      border-radius: 4px;
      font-size: 11px;
      color: #a5f3fc;
      overflow-x: auto;
    }

    .empty-state {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 300px;
      color: #6b7280;
      font-size: 14px;
      text-align: center;
    }

    .empty-state-icon {
      font-size: 32px;
      margin-bottom: 8px;
      opacity: 0.5;
    }

    @keyframes fadeIn {
      from {
        opacity: 0;
        transform: translateY(-5px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }

    /* Scrollbar styling */
    .console-container::-webkit-scrollbar {
      width: 8px;
    }

    .console-container::-webkit-scrollbar-track {
      background: #1e1e1e;
    }

    .console-container::-webkit-scrollbar-thumb {
      background: #4a4a4a;
      border-radius: 4px;
    }

    .console-container::-webkit-scrollbar-thumb:hover {
      background: #5a5a5a;
    }
  `;

  override render() {
    if (!this.messages || this.messages.length === 0) {
      return html`
        <div class="console-container">
          <div class="empty-state">
            <div>
              <div class="empty-state-icon">🖥️</div>
              <div>Console is empty</div>
            </div>
          </div>
        </div>
      `;
    }

    return html`
      <div 
        class="console-container" 
        @scroll=${this.handleScroll}
      >
        ${this.messages.map((msg, index) => this.renderMessage(msg, index))}
      </div>
    `;
  }

  private renderMessage(message: ConsoleMessage, index: number) {
    const levelClass = message.level.toLowerCase();
    const icon = this.getLevelIcon(message.level);
    const timestamp = this.formatTimestamp(message.timestamp);

    return html`
      <div class="console-message">
        <div class="message-level ${levelClass}">${icon}</div>
        <div class="message-content">
          <div>${this.escapeHtml(message.message)}</div>
          ${message.data ? html`<div class="message-data">${this.formatData(message.data)}</div>` : ''}
          ${message.source !== 'event_loop' ? html`<div class="message-source">${message.source}</div>` : ''}
        </div>
        <div class="message-timestamp">${timestamp}</div>
      </div>
    `;
  }

  private getLevelIcon(level: ConsoleLevel): string {
    const icons: Record<ConsoleLevel, string> = {
      Log: '○',
      Info: 'ℹ',
      Warn: '⚠',
      Error: '✕',
      Debug: '🐛',
      Trace: '📍',
      Table: '📊',
      Dir: '📁',
      Group: '📂',
      GroupCollapsed: '📂',
      GroupEnd: '',
    };
    return icons[level] || '○';
  }

  private formatTimestamp(isoString: string): string {
    try {
      const date = new Date(isoString);
      const hours = date.getHours().toString().padStart(2, '0');
      const minutes = date.getMinutes().toString().padStart(2, '0');
      const seconds = date.getSeconds().toString().padStart(2, '0');
      const milliseconds = date.getMilliseconds().toString().padStart(3, '0');
      return `${hours}:${minutes}:${seconds}.${milliseconds}`;
    } catch {
      return '';
    }
  }

  private formatData(data: unknown): string {
    try {
      if (typeof data === 'object' && data !== null) {
        return JSON.stringify(data, null, 2);
      }
      return String(data);
    } catch {
      return '[Unable to parse data]';
    }
  }

  private escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  private handleScroll() {
    // Track scroll position if needed
  }

  /**
   * Scroll to bottom of console
   */
  scrollToBottom(): void {
    if (this.consoleContainer) {
      this.consoleContainer.scrollTop = this.consoleContainer.scrollHeight;
    }
  }

  /**
   * Clear all messages
   */
  clear(): void {
    this.messages = [];
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'el-console-output': ConsoleOutputComponent;
  }
}
