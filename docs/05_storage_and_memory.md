# Capa 2: Almacenamiento Volátil y Gestión de Llaves

Esta capa define la persistencia de los datos y el ciclo de vida del material criptográfico en el dispositivo local. El objetivo primordial es garantizar que, sin el PIN del usuario, el material almacenado sea indistinguible de datos aleatorios y matemáticamente inaccesible mediante técnicas forenses tradicionales.

## 1. El Mecanismo de Acceso "Vault-Key"

SignalFlow utiliza un esquema de Key Wrapping (Envoltorio de Llave) para separar la autenticación del usuario de la verdadera llave de cifrado de la base de datos.

- **Derivación de Llave (Argon2id/PBKDF2):** El PIN ingresado por el usuario no se almacena en ninguna forma. Se utiliza como entropía inicial para un algoritmo de estiramiento de llave que consume recursos significativos de CPU/RAM, lo que incrementa exponencialmente el costo temporal de ataques de fuerza bruta.

- **Secreto Maestro (Master Key):** Una llave AES-256 de alta entropía generada aleatoriamente que es la encargada de cifrar la base de datos de chats y metadatos.

- **Wrapped Master Key (WMK):** El Secreto Maestro se almacena cifrado mediante la llave derivada del PIN. Sin el PIN correcto para derivar la llave de apertura, el Secreto Maestro permanece en un estado de alta entropía (ruido).

## 2. Invisibilidad y Esteganografía (Stealth Storage)

Para frustrar la identificación del material criptográfico por parte de analistas forenses, SignalFlow implementa técnicas de ocultamiento que evitan patrones de archivos sospechosos.

- **Inyección en Binarios (Resource Append):** El WMK se inyecta mediante offsets específicos (posiciones de bytes predefinidas) dentro de archivos de biblioteca (.dll) o ejecutables legítimos de la aplicación. Para el sistema operativo, el archivo conserva su firma y funcionalidad original.

- **Archivos de Ruido (Decoys):** La aplicación genera múltiples archivos de "caché" y "logs" falsos con tamaños variables. La llave real reside camuflada dentro de uno de estos archivos, mimetizándose con datos binarios genéricos para evitar el análisis de entropía simple.

- **Integridad Referencial (HMAC):** Se utiliza un código HMAC-SHA256 para firmar el archivo de la llave. Si un agente externo modifica un solo bit del archivo (intento de manipulación), el sistema detecta la inconsistencia durante el arranque y activa protocolos de defensa.

## 3. Rotación Dinámica y Volatilidad

La seguridad de los datos en reposo no es estática; se renueva en cada sesión para garantizar la confidencialidad persistente.

- **Session Key Cycling:** Cada vez que el usuario cierra la aplicación de forma segura, el sistema genera un nuevo Secreto Maestro aleatorio.

- **Re-cifrado en Caliente:** Antes de finalizar el proceso, la aplicación descifra la base de datos local con la llave antigua y la vuelve a cifrar con la nueva llave generada, moviendo el nuevo WMK a una ubicación física o offset diferente en el almacenamiento.

- **Zero-Persistence RAM:** El Secreto Maestro descifrado reside únicamente en segmentos de memoria protegida. Se ejecuta una función de Wiping (sobreescritura con ceros) inmediatamente antes de cerrar el proceso o ante cualquier señal de pánico del Sentinel Daemon.

## 4. Bomba Lógica Anti-Fuerza Bruta

Para proteger la "Bóveda" contra ataques automatizados de adivinación de PIN en el entorno local:

- **Contador de Intentos Persistente:** El sistema mantiene un registro de intentos fallidos integrado en la estructura del material criptográfico.

- **Autodestrucción de Llave (Cryptographic Erasure):** Tras alcanzar el límite de intentos (configurable entre 3 y 5), el Sentinel Daemon ejecuta una purga irreversible del archivo o segmento que contiene el Secreto Maestro Envuelto.

- **Resultado Forense:** Al destruir la llave maestra, la base de datos de chats permanece físicamente en el disco pero se vuelve matemáticamente imposible de descifrar, eliminando cualquier posibilidad de recuperación de datos.

## 5. Resumen de Seguridad:

"Esta capa transforma el almacenamiento estático en un objetivo dinámico e invisible. Al rotar las llaves en cada cierre y ocultar su ubicación mediante esteganografía, SignalFlow obliga al atacante no solo a intentar romper un cifrado robusto, sino a encontrar primero qué bits entre gigabytes de datos representan la llave real".
