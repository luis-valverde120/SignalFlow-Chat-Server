# Capa 1: Seguridad Física y Disparadores de Hardware

Esta es la capa más interna y extrema de la arquitectura de SignalFlow. Está diseñada para mitigar el escenario de "Extracción en Vivo" (_Live Extraction_), donde un adversario (ej. autoridades, atacantes físicos) obtiene control del dispositivo mientras este se encuentra encendido, desbloqueado y con las llaves criptográficas cargadas en la memoria RAM.

El objetivo de esta capa no es prevenir el acceso físico, sino garantizar la destrucción instantánea del material criptográfico (Capa 2) antes de que el adversario pueda extraerlo.

## 1. Hardware Kill Cord (Cordón de Desconexión Físico)

Para entornos de alto riesgo, SignalFlow implementa un mecanismo de "Hombre Muerto" (_Dead Man's Switch_) vinculado a periféricos de hardware.

### 1.1. Directrices de Configuración Segura (OpSec)

Para la implementación del **Hardware Kill Cord**, el usuario debe configurar manualmente en el _Sentinel Daemon_ el ID de hardware del dispositivo que servirá como ancla. Para garantizar la integridad del entorno físico, SignalFlow impone la siguiente regla de Seguridad Operativa (OpSec):

- **Periféricos Recomendados:** Se aconseja utilizar dispositivos de almacenamiento masivo (Pendrives USB genéricos), llaves de seguridad FIDO (YubiKey), o periféricos de puntero (Mouse dedicado) como dispositivos de anclaje.
- **Prohibición de Teclados (Anti-Keylogger):** Se desaconseja y restringe activamente el uso de teclados o cualquier dispositivo de Interfaz de Intercambio de Texto (HID Keyboard) como cordón de seguridad.
- **Justificación de Riesgo:** Los teclados son el vector principal de los _Hardware Keyloggers_ (implantados físicamente en la placa o en el cable). Si el atacante ha comprometido el teclado físico, podría registrar el _Master PIN_ o el _Duress PIN_ justo antes de que el dispositivo sea extraído, eludiendo las defensas de la Capa 1 y la Capa 3 simultáneamente.

## 2. Protocolo de Coacción (Plausible Deniability)

Si el usuario es coaccionado físicamente para desbloquear la aplicación, no puede negarse sin sufrir consecuencias. Para esto, se implementa una interfaz de Negación Plausible.

- **Duress PIN (PIN de Pánico):** El usuario configura un segundo PIN que luce y funciona exactamente igual que el PIN real.
- **Aislamiento de Bóveda:** Al ingresar el Duress PIN, el sistema no lanza un error. En su lugar, el algoritmo de derivación de llaves abre una "Bóveda Señuelo" pre-configurada con chats falsos o vacíos, aparentando un desbloqueo exitoso.
- **Ejecución Silenciosa:** En segundo plano, el ingreso del Duress PIN actúa como un disparador que instruye al Sentinel a destruir la verdadera "Bóveda Principal" (Capa 2) y purgar la RAM, sin dar ningún _feedback_ visual al atacante.

## 3. El "Death Signal" (Señal de Compromiso de Nodo)

La seguridad de un individuo compromete a la red. Si el nodo de Alice cae, Bob debe saberlo inmediatamente para dejar de enviarle información sensible.

- **Transmisión UDP de Emergencia:** Al activarse cualquier disparador de hardware o el Duress PIN, antes de destruir las llaves, el cliente utiliza sus últimos milisegundos de acceso a la red para emitir un "Death Signal".
- **Broadcast Cifrado:** Se envía un paquete pre-computado y firmado a los contactos activos a través de la Capa 5.
- **Reacción de los Nodos:** Cuando Bob recibe el "Death Signal" de Alice, la aplicación de Bob bloquea automáticamente la conversación, elimina las llaves de sesión compartidas (Double Ratchet) e invalida el _Fingerprint_ criptográfico de Alice, asumiendo que su dispositivo es ahora un nodo hostil.

## 4. Secuencia del Protocolo de Autodestrucción (Panic Mode)

Cuando la Capa 1 es detonada por cualquiera de los triggers mencionados, el Sentinel ejecuta la siguiente cascada de comandos irreversibles:

1.  **Network Kill:** Cierre inmediato de todos los sockets WebSockets y conexiones WebRTC para evitar fugas de datos en curso.
2.  **Death Signal Broadcast:** Envío de la alerta de compromiso a la red.
3.  **Cryptographic Erasure (Disco):** Sobrescritura aleatoria (Wiping) del sector del disco duro o del _offset_ esteganográfico que contiene el _Wrapped Master Key_ (WMK).
4.  **RAM Wiping:** Sobrescritura de las variables del proceso en memoria con ceros para destruir las llaves de sesión en texto plano.
5.  **Kernel Panic / Process Kill:** Cierre abrupto del proceso de la aplicación (`SIGKILL`) para devolver el control al sistema operativo sin dejar rastros en el archivo de paginación (_pagefile_).

---

> **Resumen Arquitectónico:** "La Capa 1 asume que el sistema operativo ha caído ante la fuerza física. Convierte el dispositivo en una granada de humo lógica: si se altera su estado físico o se aplica coerción, el sistema colapsa sobre sí mismo, protegiendo a la red y dejando al adversario con un dispositivo criptográficamente estéril".
