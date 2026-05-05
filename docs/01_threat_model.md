# Modelado de Amenazas y Perfil de Atacantes (Threat Model)

## 1. Activos a Proteger (Assets)

¿Qué es exactamente lo que tiene valor en nuestra aplicación y debemos defender a toda costa?

- **1.1. Identidad de los Nodos (Metadatos):** La información sobre _quién_ está hablando con _quién_, y desde qué direcciones IP (públicas o locales).
- **1.2. Contenido de las Comunicaciones (Mensajes):** El texto plano de las conversaciones en curso.
- **1.3. Material Criptográfico:** Las llaves privadas de sesión y la llave maestra (Header LUKS) que permite el acceso a la base de datos local.
- **1.4. Historial (Datos en Reposo):** Los mensajes almacenados temporalmente en el dispositivo físico.

---

## 2. Perfil del Adversario (Threat Actors)

Definimos tres niveles de adversarios, desde los más comunes hasta los más sofisticados, contra los cuales SignalFlow implementa contramedidas.

### Nivel 1: El Analista de Red

En este nivel buscamos proteger al usuario de ataques que puedan comprometer su privacidad y seguridad en la red, o que puedan comprometer los dispositivos mediante un ataque Man-in-the-Middle (MITM).

- **Capacidad:** El atacante puede interceptar el tráfico de red entre el cliente y el servidor.
- **Objetivo:** Leer los mensajes o descubrir la ubicación IP del servidor central/clientes.
- **Nuestra Defensa:** Cifrado Double Ratchet (Capa 4) aqui buscamos evitar que el atacante pueda leer los mensajes mediante este cifrado para esto debemos establecer un canal de comunicacion segurda entre los usuarios. Y ademas implementamos Enrutamiento vía Tor Hidden Services para evitar que el atacante pueda descubrir la ubicación IP del servidor central/clientes.

### Nivel 2: El Intruso Físico Básico

En este nivel tenemos atacantes que obtienen acceso al dispositivo fisico (computadora, smartphone, tablet, etc.) que corre SignalFlow. Este puede ser cuando este el dispositivo apagado o bloqueado.

- **Capacidad:** Los atacantes obtienen acceso físico al dispositivo cuando está apagado o bloqueado.
- **Objetivo:** Extraer el disco duro o clonar la memoria flash para leer el historial.
- **Nuestra Defensa:** Cifrado LUKS/FDE y Contraseña de usuario fuerte (Capa 2) evitando que puedan acceder a la información almacenada en el dispositivo, para esto usamos un deamon que bloquea el dispositivo fisicamente si detecta que esta siendo manipulado.

### Nivel 3: El Atacante OpSec Avanzado

Este nivel el atacante busca obtener acceso al dispositivo cuando este esta encendido para sacar la informacion del dispositivo, buscando alejar al usuario de su dispositivo y obtener acceso a el por la fuerza.

- **Capacidad:** El atacante extrae el dispositivo mientras está encendido y desbloqueado, o ejerce coerción física sobre el usuario para obtener contraseñas. Intenta conectar depuradores (Debuggers) o extraer volcados de memoria (RAM Dumps).
- **Objetivo:** Leer los mensajes en pantalla, extraer llaves criptográficas directamente de la RAM, o forzar al usuario a revelar sus contactos.
- **Nuestra Defensa:**
  - **Anti-Coerción:** _Duress Password_ que despliega un entorno falso y purga llaves reales.
  - **Extracción en Vivo:** _Sentinel Daemon_ que detecta manipulación del SO y ejecuta bloqueo criptográfico.
  - **Robo Abrupto:** _Hardware Kill Cord_ (Desconexión USB) que activa el protocolo de pánico.

---

## 3. Vectores de Ataque y Mitigación (STRIDE Methodology)

| Vector de Ataque                        | Descripción del Riesgo                                        | Contramedida en SignalFlow                                                            | Capa de Defensa |
| :-------------------------------------- | :------------------------------------------------------------ | :------------------------------------------------------------------------------------ | :-------------- |
| **Spoofing** (Suplantación)             | Un atacante se hace pasar por el Servidor de Señalización.    | El cliente solo se conecta mediante direcciones criptográficas `.onion` autenticadas. | Capa 5          |
| **Tampering** (Manipulación)            | Intento de inyectar código mediante un depurador (GDB/Frida). | _Sentinel Daemon_ detecta el _attach_ y ejecuta la purga de RAM (Panic Mode).         | Capa 3          |
| **Information Disclosure** (Fuga de IP) | WebRTC filtra la IP local a través de _ICE Candidates_.       | Forzar _mDNS_ (Multicast DNS) y obligar el uso de Servidores TURN ciegos.             | Capa 5          |
| **Information Disclosure** (RAM Dump)   | Atacante congela la RAM para extraer llaves de sesión.        | Las llaves se escriben en _tmpfs_ y se sobrescriben inmediatamente después de usarse. | Capa 2          |
| **Coerción Física**                     | El usuario es forzado a desbloquear la aplicación.            | _Duress Password_ abre interfaz de Negación Plausible y envía _Death Signal_.         | Capa 3 & 1      |

---

## 4. Limitaciones del Modelo de Seguridad (Out of Scope)

Ningún sistema es 100% seguro. Es vital reconocer qué escenarios **no** podemos mitigar:

- **Keyloggers a Nivel de Hardware:** Si el atacante instaló un keylogger físico en el teclado del usuario antes de que instalara SignalFlow, las contraseñas estarán comprometidas.
- **Cámaras Ocultas (Shoulder Surfing):** Si el atacante graba físicamente la pantalla del usuario mientras lee los mensajes.
- **Ataques de Día Cero (0-Day) en el SO:** Si el Kernel de Linux, Android o iOS está comprometido desde la base por un actor estatal (ej. Pegasus), la aplicación no puede garantizar la seguridad del entorno.
