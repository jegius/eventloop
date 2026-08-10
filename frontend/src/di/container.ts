/**
 * Простой DI-контейнер (Inversion of Control).
 *
 * Позволяет регистрировать сервисы по токену (ключу) и резолвить их
 * в компонентах. Поддерживает ленивую инициализацию через фабрики,
 * а также синглтоны (по умолчанию).
 *
 * Использование:
 * ```ts
 * const container = new Container();
 * container.register('interpreter', () => new InterpreterService());
 * const svc = container.resolve<InterpreterService>('interpreter');
 * ```
 */

/** Токен сервиса — строковый ключ, по которому сервис регистрируется и резолвится. */
export type ServiceToken<T = unknown> = string & { __service?: T };

/** Фабрика, создающая экземпляр сервиса. */
export type ServiceFactory<T> = (container: Container) => T;

/** Описание зарегистрированного сервиса. */
interface ServiceDefinition<T> {
  /** Фабрика создания экземпляра. */
  factory: ServiceFactory<T>;
  /** Является ли сервис синглтоном (кэшируется после первого создания). */
  singleton: boolean;
  /** Зарегистрированный экземпляр (для синглтонов). */
  instance?: T;
}

/**
 * DI-контейнер.
 *
 * Реализует паттерн Service Locator: сервисы регистрируются по токенам,
 * а потребители запрашивают их через `resolve`. Это позволяет разделить
 * создание зависимостей и их использование, упрощая тестирование и
 * замену реализаций.
 */
export class Container {
  /** Реестр зарегистрированных сервисов. */
  private readonly registry = new Map<ServiceToken, ServiceDefinition<unknown>>();

  /**
   * Регистрирует сервис в контейнере.
   *
   * @param token   Уникальный токен (ключ) сервиса.
   * @param factory Фабрика, создающая экземпляр сервиса.
   * @param options Опции регистрации (`singleton` — кэшировать экземпляр).
   */
  register<T>(
    token: ServiceToken<T>,
    factory: ServiceFactory<T>,
    options: { singleton?: boolean } = {}
  ): void {
    const { singleton = true } = options;
    this.registry.set(token, { factory, singleton });
  }

  /**
   * Резолвит (получает) экземпляр сервиса по токену.
   *
   * Для синглтонов экземпляр создаётся один раз и кэшируется.
   * Для не-синглтонов каждый вызов создаёт новый экземпляр.
   *
   * @throws Если сервис с указанным токеном не зарегистрирован.
   */
  resolve<T>(token: ServiceToken<T>): T {
    const def = this.registry.get(token);
    if (!def) {
      throw new Error(`Сервис "${token}" не зарегистрирован в DI-контейнере`);
    }

    // Для синглтонов возвращаем кэшированный экземпляр.
    if (def.singleton) {
      if (!def.instance) {
        def.instance = def.factory(this);
      }
      return def.instance as T;
    }

    // Для не-синглтонов создаём новый экземпляр при каждом вызове.
    return def.factory(this) as T;
  }

  /**
   * Проверяет, зарегистрирован ли сервис с указанным токеном.
   */
  has(token: ServiceToken): boolean {
    return this.registry.has(token);
  }

  /**
   * Удаляет зарегистрированный сервис из контейнера.
   */
  unregister(token: ServiceToken): void {
    this.registry.delete(token);
  }

  /**
   * Очищает контейнер (удаляет все сервисы и кэшированные экземпляры).
   */
  clear(): void {
    this.registry.clear();
  }
}

/**
 * Глобальный экземпляр DI-контейнера приложения.
 * Регистрация сервисов выполняется в `main.ts`.
 */
export const container = new Container();