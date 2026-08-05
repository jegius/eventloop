import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { Microtask, Macrotask, RenderTask, RAFTask, RICTask } from '../types.js';

export type QueueType = 'microtask' | 'macrotask' | 'render' | 'raf' | 'ric';

/**
 * Web Component for visualizing Task Queues
 */
@customElement('el-task-queue')
export class TaskQueueComponent extends LitElement {
  @property({ type: String })
  queueType: QueueType = 'microtask';

  @property({ type: Array })
  microtasks: Microtask[] = [];

  @property({ type: Array })
  macrotasks: Macrotask[] = [];

  @property({ type: Array })
  renderTasks: RenderTask[] = [];

  @property({ type: Array })
  rafTasks: RAFTask[] = [];

  @property({ type: Array })
  ricTasks: RICTask[] = [];

  static override styles = css`
    :host {
      display: block;
      font-family: 'JetBrains Mono', 'Fira Code', monospace;
    }

    .queue-container {
      min-height: 80px;
      padding: 12px;
      background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
      border-radius: 8px;
      border: 1px solid #4a4a6a;
      overflow-x: auto;
    }

    .queue-items {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }

    .queue-item {
      display: flex;
      flex-direction: column;
      min-width: 180px;
      max-width: 280px;
      padding: 10px 12px;
      border-radius: 6px;
      font-size: 12px;
      animation: fadeIn 0.3s ease-out;
      transition: transform 0.2s, box-shadow 0.2s;
      cursor: pointer;
    }

    .queue-item:hover {
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    }

    .queue-item.microtask {
      background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
      color: white;
    }

    .queue-item.macrotask {
      background: linear-gradient(135deg, #10b981 0%, #059669 100%);
      color: white;
    }

    .queue-item.render {
      background: linear-gradient(135deg, #ec4899 0%, #db2777 100%);
      color: white;
    }

    .queue-item.raf {
      background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
      color: white;
    }

    .queue-item.ric {
      background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);
      color: white;
    }

    .item-header {
      display: flex;
      align-items: center;
      gap: 6px;
      margin-bottom: 6px;
    }

    .item-icon {
      font-size: 14px;
    }

    .item-type {
      font-weight: 600;
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      opacity: 0.9;
    }

    .item-callback {
      flex: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      display: -webkit-box;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
      margin-bottom: 4px;
      font-size: 11px;
      opacity: 0.95;
    }

    .item-meta {
      display: flex;
      gap: 8px;
      font-size: 10px;
      opacity: 0.7;
    }

    .meta-item {
      display: flex;
      align-items: center;
      gap: 3px;
    }

    .empty-state {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 80px;
      color: #6b7280;
      font-size: 13px;
      text-align: center;
    }

    @keyframes fadeIn {
      from {
        opacity: 0;
        transform: scale(0.95);
      }
      to {
        opacity: 1;
        transform: scale(1);
      }
    }
  `;

  override render() {
    const items = this.getItems();

    if (!items || items.length === 0) {
      return html`
        <div class="queue-container">
          <div class="empty-state">
            <span>Queue is empty</span>
          </div>
        </div>
      `;
    }

    return html`
      <div class="queue-container">
        <div class="queue-items">
          ${items.map((item, index) => this.renderItem(item, index))}
        </div>
      </div>
    `;
  }

  private getItems(): Array<Microtask | Macrotask | RenderTask | RAFTask | RICTask> {
    switch (this.queueType) {
      case 'microtask':
        return this.microtasks;
      case 'macrotask':
        return this.macrotasks;
      case 'render':
        return this.renderTasks;
      case 'raf':
        return this.rafTasks;
      case 'ric':
        return this.ricTasks;
      default:
        return [];
    }
  }

  private renderItem(
    item: Microtask | Macrotask | RenderTask | RAFTask | RICTask,
    index: number
  ) {
    const type = this.getItemType(item);
    const icon = this.getIcon(type);
    const title = this.getTitle(item);
    const callback = this.getCallback(item);
    const meta = this.getMeta(item);

    return html`
      <div class="queue-item ${type}" title="${callback}">
        <div class="item-header">
          <span class="item-icon">${icon}</span>
          <span class="item-type">${title}</span>
        </div>
        <div class="item-callback">${callback}</div>
        ${meta ? html`<div class="item-meta">${meta}</div>` : ''}
      </div>
    `;
  }

  private getItemType(item: any): QueueType {
    if ('task_type' in item && (item.task_type?.includes('Promise') || item.task_type === 'QueueMicrotask')) {
      return 'microtask';
    }
    if ('task_type' in item && (item.task_type?.includes('Timeout') || item.task_type?.includes('Interval'))) {
      return 'macrotask';
    }
    if ('phase' in item) {
      return 'render';
    }
    if ('scheduled_frame' in item) {
      return 'raf';
    }
    if ('timeout_ms' in item) {
      return 'ric';
    }
    return this.queueType;
  }

  private getIcon(type: QueueType): string {
    const icons: Record<QueueType, string> = {
      microtask: '⚡',
      macrotask: '⏱️',
      render: '🎨',
      raf: '🖼️',
      ric: '💤',
    };
    return icons[type] || '📦';
  }

  private getTitle(item: any): string {
    if ('task_type' in item) {
      return String(item.task_type).replace(/([A-Z])/g, ' $1').trim();
    }
    if ('phase' in item) {
      return String(item.phase);
    }
    if ('scheduled_frame' in item) {
      return `Frame ${item.scheduled_frame}`;
    }
    if ('timeout_ms' in item) {
      return 'Idle Callback';
    }
    return 'Task';
  }

  private getCallback(item: any): string {
    if ('callback' in item) {
      const cb = String(item.callback);
      return cb.length > 50 ? cb.substring(0, 50) + '...' : cb;
    }
    if ('description' in item) {
      return String(item.description);
    }
    return '';
  }

  private getMeta(item: any) {
    const parts: string[] = [];

    if ('delay_ms' in item && item.delay_ms !== null) {
      parts.push(`${item.delay_ms}ms`);
    }

    if ('priority' in item && item.priority !== undefined) {
      parts.push(`P:${item.priority}`);
    }

    if ('timeout_ms' in item && item.timeout_ms !== null) {
      parts.push(`T:${item.timeout_ms}ms`);
    }

    if ('duration_estimate_ms' in item) {
      parts.push(`${item.duration_estimate_ms}ms`);
    }

    if (parts.length > 0) {
      return html`${parts.map(p => html`<span class="meta-item">${p}</span>`)}`;
    }

    return null;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'el-task-queue': TaskQueueComponent;
  }
}
