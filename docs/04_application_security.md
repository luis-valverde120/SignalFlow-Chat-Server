## 3. Prevención de Espionaje de Software (Spyware)

SignalFlow asume que el sistema operativo host puede estar comprometido por software de vigilancia comercial o malware. Para la versión del cliente de escritorio (Windows), se implementan medidas de defensa pasiva a nivel de interfaz y API.

### 3.1. Screen Shielding (Protección contra Capturas de Pantalla)

Para evitar que un spyware (o un troyano de acceso remoto - RAT) grabe el contenido de los chats o tome capturas de pantalla silenciosas, el cliente de SignalFlow hace uso de la API nativa de Windows.

- **Implementación:** Se invoca la función `SetWindowDisplayAffinity` pasándole el parámetro `WDA_MONITOR`.
- **Efecto:** El renderizador de ventanas de Windows (DWM) aislará la ventana. Si cualquier otro proceso intenta tomar un _screenshot_ (usando herramientas como recortes, OBS, o malware), la ventana de SignalFlow aparecerá como un rectángulo completamente negro, protegiendo los mensajes en texto plano.

### 3.2. Evasión de Keyloggers (Scrambled PIN Pad)

En lugar de implementar hooks invasivos a nivel de kernel para detectar keyloggers de software (lo cual suele generar falsos positivos en los antivirus), SignalFlow utiliza un enfoque de evasión por interfaz.

- **Implementación:** Para el ingreso de la Contraseña Maestra y la Contraseña de Coacción (Duress), se desactiva el uso del teclado físico.
- **Efecto:** Se presenta un teclado numérico/alfanumérico virtual en pantalla cuyos botones cambian de posición aleatoriamente en cada inicio de sesión. Esto anula la eficacia de los keyloggers (que esperan pulsaciones de hardware) y mitiga los rastreadores de clics de ratón, ya que las coordenadas de los números nunca son las mismas.
