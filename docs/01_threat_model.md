# Modelado de Amenazas y Delimitación de Alcance (Threat Model & Scope)

> **Nota del Arquitecto:** SignalFlow es un prototipo avanzado (_Proof of Concept_) desarrollado para demostrar patrones de arquitectura de grado misión-crítica. Como tal, este documento define límites estrictos sobre lo que el sistema intenta mitigar y lo que se acepta como riesgo residual (_Out of Scope_) debido a los límites de un entorno de desarrollo no empresarial.

## 1. Activos a Proteger (Assets)

Los elementos centrales que la arquitectura de SignalFlow defiende a toda costa son:

- **1.1. Metadatos Topológicos:** Información sobre quién habla con quién, cuándo y desde qué dirección IP.
- **1.2. Contenido en Tránsito:** El texto plano de las comunicaciones viajando por la red.
- **1.3. Contenido en Reposo:** El historial de mensajes almacenado físicamente en el disco local.
- **1.4. Material Criptográfico Volátil:** Las llaves privadas en la memoria RAM necesarias para descifrar la bóveda local o los mensajes.

---

## 2. Perfil del Adversario y Alcance Defensivo (Threat Actors)

Definimos los niveles de adversarios contra los cuales SignalFlow implementa contramedidas tácticas.

### Nivel 1: El Analista de Red Pasivo/ISP (✅ Totalmente Mitigado)

El atacante controla la red local, el ISP o es un nodo malicioso intentando interceptar tráfico.

- **Objetivo:** Leer mensajes, perfilar horarios de actividad o descubrir la IP de los usuarios.
- **Nuestra Defensa:** Arquitectura _Tor-Only_ obligatoria y _Blind Dialing_. El uso del protocolo _Double Ratchet_ (Capa 4) asegura la confidencialidad, mientras que la red `.onion` (Capa 5) y la ofuscación de tiempos (_Jitter_) hacen inútil el análisis estadístico de tráfico.

### Nivel 2: Intruso Físico en Reposo (✅ Totalmente Mitigado)

El dispositivo es confiscado, robado o analizado mientras está **apagado o la aplicación está cerrada**.

- **Objetivo:** Extraer el disco duro, clonar la memoria flash y aplicar análisis forense para leer el historial.
- **Nuestra Defensa:** Cifrado de base de datos a nivel de aplicación con **SQLCipher** (AES-256) y derivación de llaves con **Argon2id** (Capa 2). Políticas destructivas de SQLite (`PRAGMA secure_delete = FAST`) garantizan que los mensajes borrados sean irrecuperables.

### Nivel 3: Atacante Físico Activo / Coerción (⚠️ Parcialmente Mitigado - Alcance Delimitado)

El dispositivo es extraído mientras está desbloqueado (_Snatch-and-Grab_ / _Live-Extraction_), o el usuario es coaccionado bajo amenaza física.

- **Objetivo:** Extraer llaves directamente de la RAM o forzar la entrega de la contraseña.
- **Nuestra Defensa:** - **Contra Coerción:** _Duress PIN_ (Negación Plausible) que despliega una interfaz limpia y purga el material sensible.
  - **Contra Snatch-and-Grab:** _Hardware Kill Cord_ (Ej. un script de Rust que monitorea la desconexión de un USB específico) para detonar una sobreescritura de memoria con `zeroize`.
  - **Límite del Alcance:** _No garantizamos_ la defensa contra ataques DMA (Direct Memory Access) a través de hardware especializado, ni contra herramientas forenses que congelen la RAM criogénicamente antes de que el Kill Cord reaccione.

---

## 3. Vectores de Ataque y Mitigación (STRIDE Methodology)

| Vector de Ataque             | Descripción del Riesgo                                             | Contramedida en SignalFlow                                                                                                                                       |
| :--------------------------- | :----------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Spoofing** (Suplantación)  | Un nodo se hace pasar por un destinatario.                         | Identidades atadas a llaves criptográficas de enrutamiento `.onion` (v3). No hay PKI centralizada que pueda ser vulnerada.                                       |
| **Tampering** (Manipulación) | Modificación de la base de datos local para incriminar al usuario. | **Negación Plausible Práctica**: El cliente permite editar mensajes recibidos intencionalmente, invalidando el valor probatorio de la BBDD local en un tribunal. |
| **Info Disclosure** (Red)    | Análisis de tráfico revela correlación temporal de mensajes.       | P2P estricto asíncrono. Los mensajes se encolan (_Store-and-Forward_) y se inyecta entropía temporal (_Jitter_) en los reintentos automáticos.                   |
| **Info Disclosure** (RAM)    | Extracción de memoria revela llaves de sesión.                     | Uso riguroso de la librería `zeroize` en Rust para destruir el material criptográfico tan pronto sale del contexto de ejecución.                                 |

---

## 4. Riesgos Aceptados y Fuera de Alcance (Out of Scope)

Para mantener la viabilidad del proyecto bajo el paradigma de _Seguridad del Lado de la Aplicación (App-Level Security)_, los siguientes escenarios se declaran explícitamente fuera de alcance:

1. **Demonios Anti-Tampering a Nivel de SO (Kernel Rootkits):** Detectar si un atacante adjunta un depurador (GDB/Frida) o virtualiza el entorno requiere permisos de bajo nivel y controladores de Kernel personalizados. SignalFlow asume que el sistema operativo subyacente _no_ está comprometido por el atacante en tiempo de ejecución.
2. **Ataques de Día Cero (0-Day) en el SO o Malware (Spyware/Pegasus):** Si el dispositivo host ya está infectado con un troyano de acceso remoto (RAT) con permisos de administrador, o hay un _keylogger_ a nivel de hardware, la seguridad de la aplicación es irrelevante.
3. **Observadores Globales Pasivos (Global Passive Adversaries):** No mitigamos ataques teóricos donde una entidad (Ej. la NSA) tenga visibilidad simultánea sobre la totalidad de los nodos de la red Tor a nivel global.
4. **Protección contra Shoulder Surfing:** Grabación visual (mediante cámaras de seguridad o en persona) de la pantalla del usuario mientras la aplicación está abierta.
