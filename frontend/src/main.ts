/**
 * Точка входа приложения.
 * Регистрирует сервисы в DI-контейнере и все Web Components.
 */

// Регистрация сервисов в DI-контейнере.
// ВАЖНО: этот импорт должен быть ПЕРВЫМ, до импорта компонентов,
// чтобы сервисы были зарегистрированы до создания (upgrade) custom elements.
import './services/register';

// Импорт компонентов (регистрируются при импорте)
import './components/code-editor/code-editor';
import './components/console/console';
import './components/call-stack/call-stack';
import './components/queue/queue';
import './components/render-phases/render-phases';
import './components/execution-indicator/execution-indicator';
import './components/event-loop-visualizer/event-loop-visualizer';

// Полифиллы для Web Components (для старых браузеров)
import '@webcomponents/webcomponentsjs';

// Инициализация приложения
document.addEventListener('DOMContentLoaded', () => {
  console.log('⚡ Event Loop Visualizer запущен');
});