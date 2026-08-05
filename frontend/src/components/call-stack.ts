import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { StackFrame } from '../types.js';

/**
 * Web Component for visualizing the Call Stack
 */
@customElement('el-call-stack')
export class CallStackComponent extends LitElement {
  @property({ type: Array })
  frames: StackFrame[] = [];

  static override styles = css`
    :host {
      display: block;
      font-family: 'JetBrains Mono', 'Fira Code', monospace;
    }

    .stack-container {
      display: flex;
      flex-direction: column-reverse;
      gap: 4px;
      min-height: 200px;
      padding: 12px;
      background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
      border-radius: 8px;
      border: 1px solid #4a4a6a;
      overflow-y: auto;
    }

    .stack-frame {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 10px 12px;
      background: linear-gradient(90deg, #6366f1 0%, #8b5cf6 100%);
      border-radius: 6px;
      color: white;
      font-size: 13px;
      animation: slideIn 0.3s ease-out;
      box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
      transition: transform 0.2s, box-shadow 0.2s;
    }

    .stack-frame:hover {
      transform: translateX(4px);
      box-shadow: 0 4px 12px rgba(99, 102, 241, 0.5);
    }

    .frame-number {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 24px;
      height: 24px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 50%;
      font-size: 11px;
      font-weight: bold;
    }

    .frame-content {
      flex: 1;
      min-width: 0;
    }

    .frame-function {
      font-weight: 600;
      margin-bottom: 2px;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .frame-location {
      font-size: 11px;
      opacity: 0.8;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .frame-context {
      display: inline-block;
      padding: 2px 6px;
      background: rgba(255, 255, 255, 0.15);
      border-radius: 4px;
      font-size: 10px;
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }

    .empty-state {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 200px;
      color: #6b7280;
      font-size: 14px;
      text-align: center;
    }

    .empty-state-icon {
      font-size: 32px;
      margin-bottom: 8px;
      opacity: 0.5;
    }

    @keyframes slideIn {
      from {
        opacity: 0;
        transform: translateY(-10px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }
  `;

  override render() {
    if (!this.frames || this.frames.length === 0) {
      return html`
        <div class="stack-container">
          <div class="empty-state">
            <div>
              <div class="empty-state-icon">📚</div>
              <div>Call Stack is empty</div>
            </div>
          </div>
        </div>
      `;
    }

    return html`
      <div class="stack-container">
        ${this.frames.map((frame, index) => this.renderFrame(frame, index))}
      </div>
    `;
  }

  private renderFrame(frame: StackFrame, index: number) {
    const location = this.formatLocation(frame);
    const context = this.formatContext(frame.execution_context);

    return html`
      <div class="stack-frame">
        <div class="frame-number">${index + 1}</div>
        <div class="frame-content">
          <div class="frame-function">${this.escapeHtml(frame.function_name)}</div>
          ${location ? html`<div class="frame-location">${location}</div>` : ''}
          ${context ? html`<span class="frame-context">${context}</span>` : ''}
        </div>
      </div>
    `;
  }

  private formatLocation(frame: StackFrame): string {
    const parts: string[] = [];
    
    if (frame.file_name) {
      parts.push(frame.file_name);
    }
    
    if (frame.line_number !== null && frame.line_number !== undefined) {
      parts.push(`:${frame.line_number}`);
      if (frame.column_number !== null && frame.column_number !== undefined) {
        parts.push(`:${frame.column_number}`);
      }
    }
    
    return parts.join('');
  }

  private formatContext(context: string): string {
    // Extract context type from ExecutionContext
    if (context.startsWith('Function:') || context.startsWith('Module:')) {
      return context.split(':')[1];
    }
    return context;
  }

  private escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'el-call-stack': CallStackComponent;
  }
}
