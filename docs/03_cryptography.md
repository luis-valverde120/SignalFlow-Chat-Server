# Capa 4: Criptografía en Tránsito (E2EE)

Esta capa define el motor de cifrado de extremo a extremo (E2EE) que protege el contenido (payload) de los mensajes. Asume que la Capa 5 (Red) puede ser comprometida y que un adversario está interceptando el tráfico de red en texto cifrado.

## 1. El Protocolo: Double Ratchet Algorithm

SignalFlow no utiliza llaves estáticas simétricas (como un AES tradicional de contraseña única) ni asimétricas simples (PGP). Implementa una arquitectura basada en el protocolo _Double Ratchet_ para garantizar confidencialidad persistente incluso ante filtraciones temporales de memoria.

### 1.1. Intercambio Inicial (X3DH)

Antes de enviar el primer mensaje, los clientes realizan un intercambio de llaves _Extended Triple Diffie-Hellman (X3DH)_ a través del servidor de señalización (Capa 5).

- Esto permite que Alice le envíe un mensaje cifrado a Bob incluso si Bob está offline en ese momento (Cifrado Asíncrono).
- El resultado del X3DH es una "Llave Maestra Compartida" (Shared Secret) que inicializa las cadenas del trinquete (ratchet).

### 1.2. Ratchet Simétrico (Cadenas de Derivación)

Cada vez que Alice envía un mensaje, la llave de cifrado cambia.

- Se utiliza una Función Derivadora de Llaves (KDF), típicamente basada en HMAC-SHA256 o HKDF.
- La KDF toma la llave actual, cifra el mensaje, y escupe la llave para el _siguiente_ mensaje, destruyendo la llave anterior.
- **Propiedad de Seguridad:** _Perfect Forward Secrecy (PFS)_. Si la llave de la "Generación 50" es robada, es matemáticamente imposible aplicar ingeniería inversa a la KDF para obtener la llave de la "Generación 49". Los mensajes pasados son indescifrables.

### 1.3. Ratchet Asimétrico (Diffie-Hellman)

Cada vez que la dirección de la conversación cambia (ej. Alice termina de escribir y Bob responde), se ejecuta un nuevo intercambio Diffie-Hellman (ECDH) en segundo plano, adjunto a los metadatos del mensaje.

- Este intercambio "resetea" la KDF con entropía fresca.
- **Propiedad de Seguridad:** _Post-Compromise Security_. Si un atacante roba la llave de sesión de Alice en el T=1, perderá el acceso a la conversación en el instante en que Bob responda en el T=2, ya que la llave habrá mutado de forma impredecible para el atacante.

## 2. Autenticación y Verificación de Nodos

Para evitar ataques de intermediario (Man-in-the-Middle) donde un adversario suplanta las llaves iniciales en el servidor de señalización:

- **Fingerprints Criptográficos (Safety Numbers):** La aplicación genera un hash QR representativo de las llaves públicas de ambos clientes. Los usuarios deben verificar estos códigos fuera de banda (por ejemplo, en persona o por videollamada) para confirmar que no hay un intermediario interceptando la creación del túnel Double Ratchet.
