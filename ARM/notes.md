ARM

a profile is a set of architectures
A-profile (application profile) for high performance applications.
R-profile (real time profile) for real-time systems. 
M-profile (microcontroller profile) for microcontroller applications. 
processors are named Cortex-XN where X is A,R,M, and N is a number. Cortex here means 'brain'.
there are non-Cortex processors, but they are all outside these profiles and are older.

processors or cores known as Processing Elements (PE)

A-profile architectures include ARMv7-A and ARMv8-A and ARMv9-A
they are backward compatible
the architecture is a specification for a PE, a contract between hardware and software.
features include:
\begin{itemize}
    \item[Instruction set]
    \item[Register set]
    \item[Exception model]
    \item[Memory model]
    \item[Debug, trace, and profiling]
\end{itemize}
The architecture includes the ARM Architecture sitting on top of a System Architecture, which has various components for OSs, hypervisors, and firmware, eg Base System Architecture and Base Boot Requirements.

Build and design of a PE comes not from an architecture but a micro-architecture. 
so each Cortex-AN has a different micro-architecture





the central part of a series of processors, that is all Cortex-AN processors. 



what does apple use?