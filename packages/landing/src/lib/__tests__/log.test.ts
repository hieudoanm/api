import { afterEach, describe, expect, it, vi } from 'vitest';
import { log } from '@/lib/log';

afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllEnvs();
});

describe('log', () => {
  it('writes info messages with timestamp, color and level', () => {
    const spy = vi.spyOn(console, 'log').mockImplementation(() => {});
    log.info('hello', 42);
    expect(spy).toHaveBeenCalledOnce();
    const output = spy.mock.calls[0][0] as string;
    expect(output).toMatch(/\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/);
    expect(output).toContain('\x1b[36m[INFO]\x1b[0m');
    expect(output).toContain('hello');
    expect(output).toContain('42');
  });

  it('routes warn and error to their console methods', () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const error = vi.spyOn(console, 'error').mockImplementation(() => {});
    log.warn('careful');
    log.error('broken');
    expect(warn).toHaveBeenCalledOnce();
    expect(error).toHaveBeenCalledOnce();
    expect(warn.mock.calls[0][0]).toContain('\x1b[33m[WARN]\x1b[0m');
    expect(error.mock.calls[0][0]).toContain('\x1b[31m[ERROR]\x1b[0m');
  });

  it('serializes object arguments as JSON', () => {
    const spy = vi.spyOn(console, 'log').mockImplementation(() => {});
    log.info({ a: 1 }, 'tail');
    const output = spy.mock.calls[0][0] as string;
    expect(output).toContain('{"a":1}');
    expect(output).toContain('tail');
  });

  it('emits debug only outside production', () => {
    const debug = vi.spyOn(console, 'debug').mockImplementation(() => {});
    vi.stubEnv('NODE_ENV', 'development');
    log.debug('trace me');
    expect(debug).toHaveBeenCalledOnce();
    expect(debug.mock.calls[0][0]).toContain('\x1b[35m[DEBUG]\x1b[0m');

    debug.mockClear();
    vi.stubEnv('NODE_ENV', 'production');
    log.debug('hidden');
    expect(debug).not.toHaveBeenCalled();
  });
});
