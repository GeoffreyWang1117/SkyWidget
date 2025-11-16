import { invoke } from '@tauri-apps/api/core'

/**
 * CPU 信息接口
 */
export interface CpuInfo {
  brand: string
  core_count: number
  usage: number
  core_usage: number[]
  frequency: number
}

/**
 * 内存信息接口
 */
export interface MemoryInfo {
  total: number
  used: number
  available: number
  usage_percent: number
  swap_total: number
  swap_used: number
  swap_usage_percent: number
}

/**
 * 磁盘信息接口
 */
export interface DiskInfo {
  name: string
  mount_point: string
  file_system: string
  total_space: number
  available_space: number
  used_space: number
  usage_percent: number
  is_removable: boolean
}

export interface DisksInfo {
  disks: DiskInfo[]
  disk_count: number
  total_space: number
  total_used: number
  total_available: number
}

/**
 * 所有硬件信息
 */
export interface AllHardwareInfo {
  cpu: CpuInfo
  memory: MemoryInfo
  disk: DisksInfo
  timestamp: number
}

/**
 * 获取 CPU 信息
 */
export async function getCpuInfo(): Promise<CpuInfo> {
  return await invoke('get_cpu_info')
}

/**
 * 获取内存信息
 */
export async function getMemoryInfo(): Promise<MemoryInfo> {
  return await invoke('get_memory_info')
}

/**
 * 获取磁盘信息
 */
export async function getDiskInfo(): Promise<DisksInfo> {
  return await invoke('get_disk_info')
}

/**
 * 获取所有硬件信息（一次性获取）
 */
export async function getAllHardwareInfo(): Promise<AllHardwareInfo> {
  return await invoke('get_all_hardware_info')
}
