/**
 * 格式化字节大小为人类可读格式
 */
export function formatBytes(bytes: number): string {
  const KB = 1024
  const MB = KB * 1024
  const GB = MB * 1024
  const TB = GB * 1024

  if (bytes >= TB) {
    return `${(bytes / TB).toFixed(2)} TB`
  } else if (bytes >= GB) {
    return `${(bytes / GB).toFixed(2)} GB`
  } else if (bytes >= MB) {
    return `${(bytes / MB).toFixed(2)} MB`
  } else if (bytes >= KB) {
    return `${(bytes / KB).toFixed(2)} KB`
  } else {
    return `${bytes} B`
  }
}

/**
 * 格式化百分比
 */
export function formatPercent(value: number, decimals: number = 1): string {
  return `${value.toFixed(decimals)}%`
}

/**
 * 格式化 CPU 频率
 */
export function formatFrequency(mhz: number): string {
  if (mhz >= 1000) {
    return `${(mhz / 1000).toFixed(2)} GHz`
  }
  return `${mhz} MHz`
}
