import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { RenderPhase } from '../types.js';

/**
 * Web Component for visualizing Render Phases
 */
@customElement('el-render-phases')
export class RenderPhasesComponent extends LitElement {
  @property({ type: String })
  currentPhase: RenderPhase | null = null;

  @property({ type: Boolean })
  isAnimating: boolean = false;

  static override styles = css`
    :host {
      display: block;
      font-family: 'JetBrains Mono', 'Fira Code', monospace;
    }

    .phases-container {
      display: flex;
      flex-direction: column;
      gap: 12px;
      padding: 16px;
    }

    .phase-row {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 12px 16px;
      background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
      border-radius: 8px;
      border: 2px solid transparent;
      transition: all 0.3s ease;
    }

    .phase-row.active {
      border-color: #10b981;
      background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(16, 185, 129, 0.05) 100%);
      box-shadow: 0 0 20px rgba(16, 185, 129, 0.2);
    }

    .phase-row.completed {
      border-color: #6b7280;
      opacity: 0.6;
    }

    .phase-indicator {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 48px;
      height: 48px;
      border-radius: 50%;
      font-size: 24px;
      background: linear-gradient(135deg, #4a4a6a 0%, #5a5a7a 100%);
      transition: all 0.3s ease;
    }

    .phase-row.active .phase-indicator {
      background: linear-gradient(135deg, #10b981 0%, #059669 100%);
      animation: pulse 1.5s infinite;
      box-shadow: 0 0 20px rgba(16, 185, 129, 0.5);
    }

    .phase-info {
      flex: 1;
    }

    .phase-name {
      font-size: 14px;
      font-weight: 600;
      color: #e5e7eb;
      margin-bottom: 4px;
    }

    .phase-description {
      font-size: 12px;
      color: #9ca3af;
    }

    .phase-progress {
      width: 120px;
      height: 6px;
      background: #374151;
      border-radius: 3px;
      overflow: hidden;
    }

    .progress-bar {
      height: 100%;
      background: linear-gradient(90deg, #10b981 0%, #34d399 100%);
      border-radius: 3px;
      transition: width 0.3s ease;
      width: 0%;
    }

    .phase-row.active .progress-bar {
      animation: progressAnimation 2s infinite;
    }

    .phase-timing {
      font-size: 11px;
      color: #6b7280;
      min-width: 80px;
      text-align: right;
    }

    @keyframes pulse {
      0%, 100% {
        transform: scale(1);
        opacity: 1;
      }
      50% {
        transform: scale(1.1);
        opacity: 0.9;
      }
    }

    @keyframes progressAnimation {
      0% {
        width: 0%;
      }
      50% {
        width: 70%;
      }
      100% {
        width: 100%;
      }
    }
  `;

  private phaseDetails: Record<string, { icon: string; description: string }> = {
    Parsing: {
      icon: '📄',
      description: 'Parsing HTML and CSS source code',
    },
    Style: {
      icon: '🎨',
      description: 'Computing CSSOM and matching styles',
    },
    Layout: {
      icon: '📐',
      description: 'Calculating positions and dimensions (reflow)',
    },
    Paint: {
      icon: '🖌️',
      description: 'Filling pixels (rasterization)',
    },
    Composite: {
      icon: '🔀',
      description: 'Combining layers for final output',
    },
    Idle: {
      icon: '💤',
      description: 'Browser is idle, processing RIC callbacks',
    },
  };

  override render() {
    const phases: Array<RenderPhase> = ['Parsing', 'Style', 'Layout', 'Paint', 'Composite', 'Idle'];

    return html`
      <div class="phases-container">
        ${phases.map((phase, index) => this.renderPhase(phase, index))}
      </div>
    `;
  }

  private renderPhase(phase: RenderPhase, index: number) {
    const details = this.phaseDetails[phase];
    const isActive = this.currentPhase === phase;
    const isCompleted = this.isPhaseCompleted(phase);

    const className = [
      'phase-row',
      isActive ? 'active' : '',
      isCompleted ? 'completed' : '',
    ].join(' ');

    return html`
      <div class="${className}">
        <div class="phase-indicator">${details.icon}</div>
        <div class="phase-info">
          <div class="phase-name">${phase}</div>
          <div class="phase-description">${details.description}</div>
        </div>
        <div class="phase-progress">
          <div class="progress-bar" style="width: ${isActive ? 'auto' : (isCompleted ? '100%' : '0%')}"></div>
        </div>
        <div class="phase-timing">${this.getTiming(phase)}</div>
      </div>
    `;
  }

  private isPhaseCompleted(phase: RenderPhase): boolean {
    if (!this.currentPhase) return false;
    
    const phaseOrder: RenderPhase[] = ['Parsing', 'Style', 'Layout', 'Paint', 'Composite', 'Idle'];
    const currentIndex = phaseOrder.indexOf(this.currentPhase);
    const phaseIndex = phaseOrder.indexOf(phase);
    
    return phaseIndex < currentIndex;
  }

  private getTiming(phase: RenderPhase): string {
    // Simulated timing estimates
    const timings: Record<RenderPhase, string> = {
      Parsing: '~5-20ms',
      Style: '~3-15ms',
      Layout: '~2-10ms',
      Paint: '~5-30ms',
      Composite: '~1-5ms',
      Idle: 'variable',
    };
    return timings[phase] || '';
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'el-render-phases': RenderPhasesComponent;
  }
}
