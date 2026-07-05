 🔷 ANITRAX — ZK-Enabled Agentic Financial System

> ANITRAX es un sistema móvil de transferencias financieras peer-to-peer por Bluetooth con registro en blockchain Ethereum Sepolia y **Casper Testnet**, potenciado por un agente de inteligencia artificial controlado por voz y reforzado con Zero-Knowledge Proofs para privacidad total de usuario, identidad y transacciones.

[![Android](https://img.shields.io/badge/Android-Kotlin-green?logo=android)](https://developer.android.com)
[![Casper](https://img.shields.io/badge/Casper-Testnet-orange?logo=casper)](https://testnet.cspr.live)
[![Blockchain](https://img.shields.io/badge/Ethereum-Sepolia-blue?logo=ethereum)](https://sepolia.etherscan.io)
[![ZK](https://img.shields.io/badge/Zero--Knowledge-Privacy-purple)](#)
[![AI](https://img.shields.io/badge/AI-Gemini-orange?logo=google)](https://ai.google.dev)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)

- 🌐 Links

| Recurso | URL |
|---|---|
| 🌍 Página Web | https://franciscoalmarazlap-cmd.github.io/ANITRAWEBZK.github/ |
| 🔗 Repositorio | https://github.com/franciscoalmarazlap-cmd/anitrax-zk.github.com |
| 🎥 Demo en YouTube | https://youtu.be/QVREoHfyEoE?si=tzbN9HhUlUpzwUVQ |
| 🟠 Wallet Casper Testnet | https://testnet.cspr.live/account/02021ece4a3a164adff5b5a1a6614c451dd562783e6ede022cd79d088a63cd66d37f |
| ⛓ Contrato Sepolia | https://sepolia.etherscan.io/address/0x29f4e6B5Fcb0Ce294eA20bEcd7a7900C7E4e771F |


 🚀 ¿Qué es ANITRAX?

ANITRAX redefine las transferencias financieras móviles combinando:

- 📡 Comunicación peer-to-peer sin internet (Bluetooth)
- 🟠 Registro de eventos on-chain en **Casper Testnet**
- ⛓ Registro de transacciones en Ethereum Sepolia
- 🤖 Control total mediante inteligencia artificial por voz
- 🔐 Privacidad avanzada mediante Zero-Knowledge Proofs
- 🌐 Soporte **multi-red** (Casper, Sepolia, Polygon, Base)

El sistema permite que los usuarios realicen y verifiquen operaciones financieras sin revelar datos sensibles, manteniendo integridad criptográfica en cada capa del sistema.

 🟠 Casper Testnet — Integración Agentic

ANITRAX registra automáticamente eventos del agente IA en **Casper Testnet** como parte de su funcionamiento normal. La blockchain no es un complemento — es un componente central del sistema.

 Eventos registrados on-chain en Casper:

| Evento | Descripción |
|---|---|
| `transfer_sent` | Transferencia Bluetooth exitosa |
| `transfer_received` | Transferencia recibida |
| `task_start` | Inicio de sesión de transferencia |
| `task_end` | Transferencia completada |
| `agent_execution` | El agente IA ejecuta una acción |
| `zk_proof_generated` | Prueba ZK de solvencia generada |
| `bluetooth_connected` | Conexión Bluetooth establecida |
| `critical_event` | Error crítico del sistema |
| `voice_command_executed` | Comando de voz procesado |

 ¿Cómo funciona?

ANITRAX ejecuta acción
       ↓
Genera hash SHA-256 del evento
       ↓
CasperManager.kt → POST /casper/registrar-evento
       ↓
FastAPI (puerto 8001) genera deploy hash
       ↓
Registro en Casper Testnet
       ↓
Verificable en testnet.cspr.live
```

 Wallet ANITRAX en Casper Testnet:
```
02021ece4a3a164adff5b5a1a6614c451dd562783e6ede022cd79d088a63cd66d37f
Saldo: 10.32 CSPR
```

> Solo el hash SHA-256 del evento se almacena on-chain — nunca información privada del usuario.

 🔐 Core Innovation — Zero-Knowledge Privacy Layer

ANITRAX implementa Zero-Knowledge Proof como capa central de privacidad para transformar un sistema financiero tradicional en un sistema verificable sin exposición de datos.

¿Qué protege el ZK en ANITRAX?

🔒 1. Identidad privada
El usuario puede autenticarse sin revelar nombre, credenciales ni datos biométricos crudos. Solo se prueba: *"usuario válido"*.

💰 2. Fondos sin exposición
Se verifica que el usuario tiene saldo suficiente sin mostrar balance total ni historial financiero completo.

🤖 3. Acciones del agente IA
El sistema ejecuta comandos por voz e interpreta intenciones financieras sin que los inputs sensibles del usuario se expongan públicamente.

🟠 4. ZK + Casper on-chain
El hash SHA-256 de cada prueba ZK se registra en Casper Testnet — verificable, inmutable, sin datos sensibles.

🔗 5. Integridad de transacciones
Se prueba que la transacción es válida, no fue manipulada y cumple las reglas del sistema — sin revelar detalles internos innecesarios.

 ⚙️ Arquitectura completa del sistema
┌─────────────────────────────────────────────────────┐
│                   ANITRAX Android App                │
│                                                      │
│  🎤 Voice Input → 🤖 Gemini AI Agent                 │
│         ↓                ↓                           │
│  📡 Bluetooth P2P    🔐 ZK Proof (Circom)            │
│         ↓                ↓                           │
│  📦 TransferData ← CasperManager.kt                  │
│         ↓                ↓                           │
│  ⛓ MetaMask (Sepolia)  🟠 Casper Testnet            │
└─────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────┐
│              Servidor Unificado FastAPI              │
│                  Puerto 8001                         │
│                                                      │
│  POST /generar-prueba  → Circom + snarkjs            │
│  POST /casper/registrar-evento → Casper Testnet      │
│  GET  /casper/balance  → Saldo CSPR                  │
└─────────────────────────────────────────────────────┘
                          ↕ ngrok
               📱 Android ←→ 💻 PC Local
```

| Capa | Función |
|---|---|
| 🎤 Input Layer | Voz, UI táctil o Bluetooth P2P |
| 🤖 AI Agent Layer | Interpretación de lenguaje natural (Gemini) |
| 🔐 ZK Layer | Generación de pruebas de conocimiento cero (Circom) |
| 🟠 Casper Layer | Registro de eventos del agente on-chain |
| ⛓ Blockchain Layer | Registro en Ethereum Sepolia |
| 🔎 Verification Layer | Validación on-chain del proof |
| ⚡ Execution Layer | Ejecución de transferencia o acción |

---

## 🤖 ANITRAX AI Agent System

El sistema funciona como un agente inteligente multi-modal:

- Control por voz completo (Speech-to-Action)
- Comprensión de lenguaje natural financiero
- Ejecución de transferencias automatizadas
- Confirmación inteligente de acciones
- Respuesta por voz (Text-to-Speech)
- Modo accesible sin interacción visual
- **Registro automático de acciones en Casper Testnet**

 ⛓ Blockchain Layer

 Ethereum Sepolia
- Smart Contract Solidity para registro de transacciones
- Guarda: sender, receiver, amount, timestamp, note
- Comunicación directa vía JSON-RPC
- MetaMask Deep Link para firma segura
- Registro de hash + proof ZK en cadena

Contrato: `0x29f4e6B5Fcb0Ce294eA20bEcd7a7900C7E4e771F`

 🟠 Casper Testnet (nuevo)
- Registro de eventos del agente IA on-chain
- Hash SHA-256 del evento — sin datos sensibles
- Módulo `CasperManager.kt` integrado en Android
- Servidor puente FastAPI (puerto 8001)
- Modo demo (sin .pem) y modo live (con .pem)

Wallet: `02021ece4a3a164adff5b5a1a6614c451dd562783e6ede022cd79d088a63cd66d37f`

 🌐 Multi-red sin fricciones

El usuario agrega wallets de distintas redes **una sola vez**. Al transferir, elige qué red usar — sin cambiar configuraciones ni redes manualmente:

| Red | Símbolo | Estado |
|---|---|---|
| Ethereum Sepolia | ETH | ✅ Activo |
| Casper Testnet | CSPR | ✅ Activo |
| Polygon Amoy | MATIC | ✅ Listo |
| Base Sepolia | ETH | ✅ Listo |

---

 🔵 Peer-to-Peer Bluetooth Layer

- Transferencias sin internet
- Conexión directa entre dispositivos Android
- Envío de payload financiero estructurado con ZK hash incluido
- Servicio en segundo plano (Foreground Service)
- Reconexión automática y persistente

 🔐 Security & Authentication Model

| Tecnología | Función |
|---|---|
| BiometricPrompt | Autenticación por huella |
| Voice Authentication | Firma de voz del usuario |
| ZK Proofs (Circom) | Privacidad criptográfica |
| Bluetooth Encryption | Protección de datos P2P |
| Smart Contract Validation | Seguridad on-chain Sepolia |
| Casper On-chain Events | Auditoría inmutable del agente |

 ♿ Accessibility System (3 modos)

| Modo | Descripción |
|---|---|
| Standard | Interfaz visual completa con selector multi-red |
| Voice-First (Sinestético) | Control total sin pantalla — el agente IA guía cada paso |
| Visual-Assist | UI adaptada, botones grandes, sin audio |

 📈 DeFi Simulation Engine

- Sistema de rendimiento por niveles (2% – 8%)
- Interés compuesto mensual simulado
- Simulador de ganancias futuras
- Historial financiero estructurado
- Arquitectura lista para integración con protocolos reales (Aave / Compound)



 📁 Estructura del repositorio


ANITRAX/
├── app/
│   └── src/main/java/com/anitrax/app/
│       ├── ai/                    # OpenAIClient, SpeechEngine
│       ├── bluetooth/             # BluetoothTransferManager.kt
│       ├── casper/                # CasperManager.kt ← NUEVO
│       ├── data/models/           # TransferData, TransferRecord
│       ├── ui/
│       │   ├── base/              # BaseActivity
│       │   └── transfer/          # TransferActivity.kt
│       ├── utils/                 # PrefsManager.kt, VoucherGenerator
│       ├── web3/                  # Web3Manager, WalletActivity, MetaMaskManager
│       └── zk/                   # ZkProofClient.kt
├── server/
│   └── anitrax_server.py          # Servidor unificado ZK + Casper (FastAPI)
├── zk-circuit/
│   ├── solvencia.circom            # Circuito Zero-Knowledge
│   ├── solvencia_final.zkey        # Clave de prueba
│   └── solvencia_js/              # WASM + witness generator
├── web/
│   ├── index.html                 # Página web ANITRAX
│   ├── an1.jpg                    # Screenshot: selector multi-red
│   └── an2.jpg                    # Screenshot: transferencia Casper
└── README.md
```



 📲 Instalación de ANITRAX

 Requisitos previos

- Android 8.0 o superior (API 26+)
- Bluetooth activado
- MetaMask instalado (para Sepolia)
- Android Studio (para compilar desde código)
- Node.js + snarkjs (para el servidor ZK)
- Python 3.10+ (para el servidor FastAPI)

 Instalación desde código fuente

Paso 1: Clonar el repositorio
```bash
git clone https://github.com/franciscoalmarazlap-cmd/anitrax-zk.github.com
cd anitrax-zk.github.com
```

Paso 2: Levantar el servidor unificado (ZK + Casper)
```bash
Instalar dependencias Python
py -m pip install fastapi uvicorn pycspr

 Levantar servidor en puerto 8001
cd server
py -m uvicorn anitrax_server:app --host 0.0.0.0 --port 8001 --reload
```
Paso 3: Levantar túnel ngrok
`bash
ngrok http 8001
 Copiar la URL https://....ngrok-free.app
Actualizar BASE_URL en ZkProofClient.kt y CasperManager.kt


Paso 4: Compilar e instalar la app
- Abrir proyecto en Android Studio
- Esperar Gradle Sync
- Conectar dispositivo físico
- Presionar Run ▶

 Permisos requeridos

| Permiso | Uso |
|---|---|
| 📡 Bluetooth | Transferencias P2P |
| 🎤 Micrófono | Control por voz |
| 🔐 Biometría | Seguridad |
| 🔔 Notificaciones | Alertas en tiempo real |
| 📱 Accesibilidad | Modo sinestético |

 🧪 Prueba rápida

1. Abre ANITRAX en **2 teléfonos** con Bluetooth activado
2. En el celular 1: ve a Contactos → selecciona el celular 2
3. Ingresa monto (ej. `0.001`)
4. Toca **Enviar** → selecciona la wallet (Casper o Sepolia)
5. Confirma por voz o huella
6. Observa el servidor: verás `POST /casper/registrar-evento 200 OK`
7. Si elegiste Sepolia: firma en MetaMask
8. Verifica en [testnet.cspr.live](https://testnet.cspr.live) o [Etherscan](https://sepolia.etherscan.io)

 🏆 Por qué ANITRAX es único

 ✔ ZK no es decorativo — es estructural
- Identidad privada
- Validación de fondos sin exponer saldo
- Ejecución de acciones sin exposición de inputs

 ✔ Casper Agentic — el agente registra sus acciones
- Cada decisión del agente IA queda auditada on-chain en Casper
- Sin intervención del usuario — completamente automático

 ✔ Caso de uso real — no solo teoría
No es solo login o voting: es un **sistema financiero completo** con agente IA, Bluetooth P2P, ZK proofs y multi-red blockchain.

 ✔ Arquitectura híbrida avanzada
- AI + Casper Testnet + Ethereum + Bluetooth + ZK Proofs

 ✔ Multi-red sin fricción
- El usuario agrega su wallet una sola vez y usa cualquier red al transferir

 Notas importantes

- Solo funciona en redes de **prueba** (Sepolia + Casper Testnet)
- No usar con tokens reales
- Bluetooth debe estar visible en ambos dispositivos
- El servidor ZK + Casper debe estar corriendo en la PC durante las pruebas
- La URL de ngrok puede cambiar al reiniciar — actualizar `BASE_URL` en `ZkProofClient.kt` y `CasperManager.kt`

 📄 Licencia

MIT License — Copyright (c) 2026 Francisco Almaraz

---

*ANITRAX es una aplicación funcional desarrollada para hackathon. Todos los registros blockchain son en redes de prueba.*
