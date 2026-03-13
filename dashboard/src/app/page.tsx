"use client"

import { Shield, Cpu, Activity, HardDrive } from "lucide-react"
import {
ResponsiveContainer,
AreaChart,
Area,
Line,
XAxis,
YAxis,
Tooltip
} from "recharts"

const data = [
{time:"-60s", os:40, hw:42},
{time:"-50s", os:38, hw:44},
{time:"-40s", os:39, hw:48},
{time:"-30s", os:42, hw:55},
{time:"-20s", os:41, hw:60},
{time:"-10s", os:45, hw:72},
{time:"now", os:46, hw:78}
]

export default function Page(){

const clbdScore = 12

return(

<div className="p-6 bg-slate-950 min-h-screen text-slate-200">

{/* HEADER */}

<div className="flex justify-between mb-6">

<h1 className="text-2xl text-cyan-400 flex items-center gap-2">
<Shield/>
Latent-Sentry: CLBD Monitor
</h1>

<div className="flex gap-6 text-sm items-center">

<div className="bg-green-500/20 text-green-400 px-3 py-1 rounded animate-pulse">
eBPF Sensor Active
</div>

<div>Node IP: 10.0.0.12</div>
<div>Uptime: 12h 44m</div>

</div>

</div>

{/* KPI */}

<div className="grid grid-cols-4 gap-4 mb-6">

<KPI title="CLBD Score" value={clbdScore} icon={<Activity/>}/>

<KPI title="GPU VRAM Hijack Attempts" value={7} icon={<Cpu/>}/>

<KPI title="Protected AI Assets" value={3} icon={<HardDrive/>}/>

<KPI title="Threats Neutralized" value={12} icon={<Shield/>}/>

</div>

{/* CHART */}

<div className="bg-slate-900/50 p-4 rounded-xl mb-6">

<h2 className="text-cyan-400 mb-2">
Divergence Graph
</h2>

<div className="h-72">

<ResponsiveContainer>

<AreaChart data={data}>

<XAxis dataKey="time"/>
<YAxis/>
<Tooltip/>

<Area
type="monotone"
dataKey="hw"
stroke="#a855f7"
fill="#ef4444"
/>

<Line
type="monotone"
dataKey="os"
stroke="#06b6d4"
strokeWidth={2}
/>

</AreaChart>

</ResponsiveContainer>

</div>

</div>

{/* LOG + EBPF */}

<div className="grid grid-cols-2 gap-6">

<div className="bg-slate-900/50 p-4 rounded-xl">

<h2 className="text-cyan-400 mb-3">
Threat Enforcement Log
</h2>

<table className="w-full text-sm">

<thead className="text-slate-400">

<tr>
<th>Timestamp</th>
<th>Event</th>
<th>Hidden PID</th>
<th>Target</th>
<th>Action</th>
</tr>

</thead>

<tbody>

<tr className="text-red-400">

<td>10:37:42 AM</td>
<td>Model Exfiltration Attempt</td>
<td>PID 4091 (Hidden)</td>
<td>/models/llama_weights.pt</td>
<td>SIGKILL Executed</td>

</tr>

</tbody>

</table>

</div>

<div className="bg-black p-4 rounded-xl font-mono text-sm text-green-400">

<h2 className="text-cyan-400 mb-3">
Active Hardware Hooks
</h2>

<p>[sys_enter_read] fd=3 size=500MB → OK</p>
<p>[sys_ioctl] /dev/nvidia0 → BLOCKED</p>
<p>[sys_read] /models/model.safetensors → OK</p>

</div>

</div>

</div>

)

}

function KPI({title,value,icon}:any){

return(

<div className="bg-slate-900/50 p-4 rounded-xl">

<div className="flex justify-between">

<div>

<p className="text-slate-400 text-sm">
{title}
</p>

<p className="text-green-400 text-2xl font-bold">
{value}
</p>

</div>

<div className="text-cyan-400">
{icon}
</div>

</div>

</div>

)

}