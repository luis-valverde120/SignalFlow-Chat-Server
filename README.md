# SignalFlow: Ultra-Secure Hybrid P2P Messaging Infrastructure

> **Nota del Desarrollador:** Este proyecto se encuentra actualmente en fase de diseño de arquitectura y modelado de amenazas (Threat Modeling).

SignalFlow no es simplemente una aplicación de mensajería; es una infraestructura de comunicación de grado militar diseñada bajo los principios estrictos de **Privacy by Design** y **Zero Trust**. Su objetivo es proporcionar un anonimato absoluto y resistencia contra análisis de tráfico, incautación física y coerción.

## Características Principales (Core Features)

- **Arquitectura P2P Híbrida (WebRTC):** Un servidor de señalización ciego (_Zero-Knowledge Signaling Server_) que solo se utiliza para el intercambio inicial de tokens. Las comunicaciones fluyen de manera descentralizada de extremo a extremo.
- **Enrutamiento Tor (.onion):** Todo el tráfico entre los clientes y el servidor de señalización se enruta a través de la red Tor, ocultando las direcciones IP tanto del servidor como de los usuarios.
- **Cifrado Double Ratchet:** Implementación de cifrado E2EE asíncrono, garantizando _Perfect Forward Secrecy_ (PFS) y _Post-Compromise Security_.
- **Sentinel Daemon (Anti-Tampering):** Un proceso de bajo nivel que vigila el sistema en busca de depuradores, entornos virtualizados o extracción de hardware (Hardware Dead Man's Switch).
- **Borrado Criptográfico (Cryptographic Erasure):** Ante un evento de emergencia, el sistema no borra mensajes, sino que destruye la cabecera de cifrado (LUKS) y purga la memoria RAM (tmpfs) en milisegundos.
- **Negación Plausible (Plausible Deniability):** Soporte para contraseñas de coacción (_Duress Passwords_) que cargan un entorno de chat falso mientras envían una "Señal de Muerte" a la red en segundo plano.

## Modelo de Seguridad en Profundidad (Defense in Depth)

SignalFlow está diseñado en 5 capas concéntricas de seguridad:

1.  **Capa Física:** Triggers de hardware y Temporizadores de Hombre Muerto.
2.  **Capa de Almacenamiento:** Ejecución "Diskless" en RAM y cifrado de reposo.
3.  **Capa de OpSec:** Sentinel Daemon y mecanismos anti-manipulación.
4.  **Capa de Criptografía:** Algoritmo Double Ratchet para datos en tránsito.
5.  **Capa de Red:** Enrutamiento multicapa vía Tor y ofuscación WebRTC.

## Stack Tecnológico (Propuesto)

- **Signaling Server:** Node.js / Python (WebSockets)
- **Client / Frontend:** _[Aquí pondrás el framework que elijas, ej: React, Flutter, CLI]_
- **Criptografía:** _[Librerías criptográficas que vayas a usar]_
- **Infraestructura:** Docker, Tor Hidden Services.

## Hoja de Ruta de Desarrollo (Roadmap)

El desarrollo está dividido en 4 fases incrementales para asegurar la estabilidad de cada capa:

- [ ] **Fase 1: El Fundamento.** Servidor de señalización básico con WebSockets (TCP/IP).
- [ ] **Fase 2: Descentralización.** Implementación de WebRTC para saltar a una arquitectura P2P pura.
- [ ] **Fase 3: La Bóveda.** Integración de criptografía asimétrica y el protocolo Double Ratchet.
- [ ] **Fase 4: Modo Paranoia.** Implementación del demonio Sentinel, soporte para Tor y borrado criptográfico.

## Descargo de Responsabilidad Legal y Ética

SignalFlow es un proyecto de investigación con fines educativos y de demostración de habilidades en Arquitectura de Seguridad. La implementación de criptografía personalizada o protocolos de evasión debe ser auditada por profesionales antes de su uso en situaciones de riesgo de vida real.

---

_Diseñado y desarrollado por Luis Valverde Rivera._
