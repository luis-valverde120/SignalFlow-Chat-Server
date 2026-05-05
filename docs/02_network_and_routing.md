# Capa 5: Enrutamiento Dinámico y Anonimato de Red

Esta capa constituye la frontera exterior de la arquitectura de SignalFlow. Asume un entorno de red inherentemente hostil, donde los Proveedores de Servicios de Internet (ISPs), agencias gubernamentales o nodos maliciosos monitorizan activamente el tráfico. El objetivo de esta capa es proteger los **metadatos** (quién habla con quién, cuándo y desde dónde) antes de que el tráfico alcance las capas de cifrado.

## 1. Topología de Red Híbrida (Tor + WebRTC)

Para resolver el dilema entre "Anonimato Absoluto" (lento) y "Comunicación en Tiempo Real" (vulnerable a fugas de IP), SignalFlow implementa una arquitectura dividida en dos fases o planos operativos:

### 1.1. Fase 1: Plano de Control y Señalización (Vía Tor)

El servidor central de SignalFlow no procesa, no enruta y no almacena mensajes. Su única función es actuar como un intermediario ciego para que dos nodos intercambien sus credenciales de conexión WebRTC (Ofertas y Respuestas SDP).

- **Tor Hidden Services (v3):** El servidor opera exclusivamente dentro de la darknet utilizando una dirección criptográfica `.onion`. A nivel de sistema operativo (Node.js/Python), el servidor está anclado estrictamente a `127.0.0.1` y no expone puertos a la red pública (Clearnet).
- **Proxy SOCKS5 Local:** Los clientes no utilizan DNS estándar para ubicar el servidor (previniendo fugas y envenenamiento DNS). Todo el tráfico inicial se enruta a través de un demonio Tor embebido en el cliente, el cual levanta un proxy SOCKS5 local.
- **Beneficio de Seguridad:** El ISP del usuario solo detecta tráfico cifrado hacia la red Tor. El Servidor de SignalFlow solo registra la IP del "Nodo de Salida" de Tor, haciendo matemáticamente imposible vincular la solicitud con la IP real del usuario.

### 1.2. Fase 2: Plano de Datos (WebRTC Ofuscado)

Una vez que las descripciones de sesión (SDP) se han intercambiado a través de la red Tor, los clientes saltan a un canal de alta velocidad (WebRTC) para la transmisión del texto cifrado por la Capa 4. Para mantener el anonimato en este salto P2P, se imponen las siguientes directivas:

- **Bloqueo de P2P Directo (Strict TURN):** La conexión Peer-to-Peer directa está deshabilitada por diseño. Para evitar que el "Cliente A" descubra la IP pública del "Cliente B", todas las conexiones se fuerzan a través de servidores de relevo TURN (_Traversal Using Relays around NAT_).
- **Ofuscación LAN (mDNS):** Las políticas de WebRTC por defecto exponen direcciones IP de red local (ej. `192.168.1.5`) mediante _ICE Candidates_. SignalFlow fuerza el uso de Multicast DNS (mDNS) para enmascarar estas IPs con hostnames temporales y anónimos (ej. `c8a7b...local`).
- **Prevención de Fugas UDP:** Para evitar que el tráfico escape de túneles VPN activos a nivel de sistema operativo (un fallo común en protocolos basados en UDP), la aplicación cliente fuerza a WebRTC a operar estrictamente sobre el protocolo TCP (`turn:server?transport=tcp`).

## 2. Contramedidas de Análisis de Tráfico (Traffic Analysis Defenses)

Incluso con el enrutamiento anonimizado, un adversario que observe pasivamente un nodo puede utilizar análisis estadístico para deducir patrones de comportamiento (por ejemplo, identificar una ráfaga de paquetes pequeños como "tecleo" frente a un paquete grande como una "imagen").

Para mitigar los ataques de inferencia, SignalFlow implementa modelado de tráfico en el túnel WebRTC:

- **Traffic Padding (Relleno de Paquetes):** El tamaño de los paquetes es uniforme. Todos los mensajes enviados a través de _WebRTC Data Channels_ se normalizan a un tamaño fijo (ej. bloques de 4 KB). Si un mensaje es menor a este tamaño, el cliente inyecta bytes aleatorios (ruido criptográfico) antes de aplicar el cifrado _Double Ratchet_.
- **Cover Traffic (Tráfico de Cobertura):** Para ocultar la temporalidad de las conversaciones, el cliente genera pulsos de red falsos (_Heartbeats_ y ruido blanco) hacia el servidor TURN a intervalos aleatorios y continuos. Esto satura el canal con actividad ilegítima, imposibilitando que un observador externo determine si el usuario está enviando un mensaje real o si la aplicación simplemente está en segundo plano.

---

> **Nota Arquitectónica:** Si la Capa 5 (Red) es vulnerada o interceptada, la confidencialidad de la conversación queda inmediatamente garantizada por la **Capa 4 (Criptografía)**.
