🟠 ANITRAX — Agentic Financial System on Casper

> **ANITRAX es el primer sistema móvil de transferencias financieras peer-to-peer que usa Casper Testnet como capa de auditoría agentic** — combinando Bluetooth sin internet, Zero-Knowledge Proofs, IA por voz y registro on-chain automático de cada acción del agente en Casper Network.

[![Casper](https://img.shields.io/badge/Casper-Testnet-orange)](https://testnet.cspr.live)
[![Android](https://img.shields.io/badge/Android-Kotlin-green?logo=android)](https://developer.android.com)
[![Blockchain](https://img.shields.io/badge/Ethereum-Sepolia-blue?logo=ethereum)](https://sepolia.etherscan.io)
[![ZK](https://img.shields.io/badge/Zero--Knowledge-Privacy-purple)](#)
[![AI](https://img.shields.io/badge/AI-Gemini-orange?logo=google)](https://ai.google.dev)
[![License](https://img.shields.io/badge/License-MIT-yellow)](LICENSE)


## 🌐 Links

| Recurso | URL |
|---|---|
| 🌍 Página Web | https://franciscoalmarazlap-cmd.github.io/ANITRAXGASPERWEB/ |
| 🟠 Wallet Casper Testnet | https://testnet.cspr.live/account/02021ece4a3a164adff5b5a1a6614c451dd562783e6ede022cd79d088a63cd66d37f |
| ⛓ Contrato Ethereum Sepolia | https://sepolia.etherscan.io/address/0x29f4e6B5Fcb0Ce294eA20bEcd7a7900C7E4e771F |
| 🎥 Demo en YouTube | https://youtu.be/QVREoHfyEoE?si=tzbN9HhUlUpzwUVQ |
| 🔗 Repositorio | https://github.com/franciscoalmarazlap-cmd/ANITRAXGASPER/tree/ANITRAX.github.io |

---

 ❌ El problema que resuelve ANITRAX

 El dinero digital tiene 3 problemas fundamentales que nadie ha resuelto juntos:

**1. Necesita internet para moverse**
Millones de personas en zonas rurales, hospitales, eventos o emergencias no pueden hacer transferencias porque no tienen señal. Las apps bancarias tradicionales simplemente no funcionan sin conexión.

**2. Los agentes de IA no tienen auditoría**
Cuando una IA toma decisiones financieras en nombre del usuario (ejecutar transferencias, verificar saldos, confirmar montos), nadie puede verificar qué hizo el agente, cuándo, ni si actuó correctamente. No existe un registro inmutable de las acciones del agente.

**3. Las transferencias exponen datos privados**
Para verificar que alguien tiene suficiente saldo, los sistemas tradicionales exponen el balance completo. Para autenticarse, exponen credenciales. No hay forma de probar "tengo suficiente" sin mostrar cuánto tienes.

 ✅ ANITRAX resuelve los 3 simultáneamente:

| Problema | Solución ANITRAX |
|---|---|
| Sin internet | Bluetooth P2P directo entre celulares |
| IA sin auditoría | Cada acción del agente registrada on-chain en **Casper Testnet** |
| Datos expuestos | Zero-Knowledge Proofs — solo se prueba solvencia, nunca el saldo real |

---
 🟠 Casper Testnet — El corazón agentic de ANITRAX

ANITRAX usa Casper Testnet como **capa de auditoría inmutable del agente IA**. Cada vez que el agente toma una decisión importante, el hash criptográfico de esa acción queda registrado on-chain en Casper — automáticamente, sin intervención del usuario.

### Contrato inteligente AnitraxRegistry (Odra/Rust)

```rust
#[odra::module(events = [EventRegistered])]
pub struct AnitraxRegistry {
    event_count: Var<u64>,
}

#[odra::event]
pub struct EventRegistered {
    pub event_id: String,
    pub event_type: String,
    pub event_hash: String,  // SHA-256 — nunca datos privados
    pub timestamp: u64,
    pub sender: Address,
}
```

El contrato recibe el hash SHA-256 de cada evento del agente y lo registra on-chain. **Nunca almacena datos privados del usuario** — solo huellas criptográficas verificables.

### Eventos que generan transacciones reales en Casper:

| Evento | Cuándo ocurre |
|---|---|
| `task_start` | El agente inicia una sesión de transferencia |
| `agent_execution` | El agente IA ejecuta una acción autónoma |
| `zk_proof_generated` | Se genera prueba ZK de solvencia |
| `bluetooth_connected` | Conexión Bluetooth establecida |
| `transfer_sent` | Transferencia completada exitosamente |
| `transfer_received` | Transferencia recibida |
| `critical_event` | El agente detecta un error crítico |
| `task_end` | El agente finaliza la tarea |
| `voice_command_executed` | Comando de voz procesado por el agente |

### ¿Cómo fluye el registro en Casper?

```
Usuario dice "envía 0.1 ETH a Juan"
              ↓
    Agente IA interpreta (Gemini)
              ↓
    Genera hash SHA-256 del evento
              ↓
    CasperManager.kt → POST /casper/registrar-evento
              ↓
    Servidor FastAPI genera deploy hash
              ↓
    AnitraxRegistry contract → Casper Testnet
              ↓
    Evento inmutable en testnet.cspr.live ✅
```

### Wallet ANITRAX activa en Casper Testnet:
```
Public Key: 02021ece4a3a164adff5b5a1a6614c451dd562783e6ede022cd79d088a63cd66d37f
Saldo:      10.32 CSPR
Red:        Casper Testnet
```

---

 🔐 Zero-Knowledge Proofs — Privacidad total

Antes de cada transferencia Bluetooth, el agente IA genera una prueba ZK de solvencia usando el circuito Circom:

```
Saldo real del usuario: 0.05 ETH  ← nunca se transmite
Monto a transferir:     0.001 ETH ← es público
Prueba ZK generada:     sha256(proof) ← solo esto sale del dispositivo
```

El receptor y la blockchain verifican que el emisor tiene fondos suficientes **sin saber cuánto tiene exactamente**. El hash de esta prueba ZK también queda registrado en Casper Testnet.

Lo que protege el ZK en ANITRAX:

**🔒 Identidad privada** — El usuario demuestra que es válido sin revelar nombre, credenciales ni datos biométricos.

**💰 Fondos ocultos** — Se verifica solvencia sin exponer el balance total ni el historial financiero.

**🤖 Acciones del agente privadas** — Los inputs del usuario al agente IA no se exponen públicamente.

**🟠 ZK + Casper** — El hash de cada prueba ZK queda registrado on-chain en Casper — verificable, inmutable, sin datos sensibles.

---

📡 Bluetooth P2P — Sin internet, sin límites

ANITRAX transfiere dinero directamente de celular a celular usando Bluetooth RFCOMM — sin WiFi, sin datos móviles, sin servidor intermediario.

**Casos de uso reales:**
- Zonas rurales sin señal
- Hospitales con redes restringidas
- Eventos masivos con redes saturadas
- Emergencias donde cae internet
- Países con infraestructura limitada

El payload de la transferencia incluye el **hash ZK** como prueba de solvencia, garantizando que el emisor tiene fondos sin exponer cuánto tiene.

---

🤖 Agente IA — Control total por voz

ANITRAX no es solo una app con IA decorativa. El agente Gemini **toma decisiones reales de forma autónoma**:

- Escucha el comando de voz del usuario
- Interpreta la intención financiera en lenguaje natural
- Selecciona la wallet y red correcta
- Genera la prueba ZK de solvencia
- Ejecuta la transferencia Bluetooth
- Registra cada acción en Casper Testnet
- Confirma el resultado por voz

**Todo esto sin que el usuario toque la pantalla.** Y cada paso queda auditado on-chain en Casper.

---

 🌐 Multi-red sin fricción

El usuario agrega sus wallets una sola vez. Al transferir, elige qué red usar — el saldo de cada wallet se muestra en tiempo real:

| Red | Símbolo | Estado |
|---|---|---|
| **Casper Testnet** | **CSPR** | ✅ **Activo** |
| Ethereum Sepolia | ETH | ✅ Activo |
| Polygon Amoy | MATIC | ✅ Listo |
| Base Sepolia | ETH | ✅ Listo |

---

⚙️ Arquitectura del sistema

```
┌──────────────────────────────────────────────────────┐
│                  ANITRAX Android App                  │
│                                                       │
│   🎤 Voz → 🤖 Gemini Agent → 📡 Bluetooth P2P        │
│                   ↓                                   │
│         🔐 ZK Proof (Circom/snarkjs)                  │
│                   ↓                                   │
│         CasperManager.kt ← TransferActivity           │
│                   ↓              ↓                    │
│         🟠 Casper Testnet  ⛓ MetaMask (Sepolia)      │
└──────────────────────────────────────────────────────┘
                      ↕ ngrok
┌──────────────────────────────────────────────────────┐
│            Servidor Unificado FastAPI                 │
│                 Puerto 8001                           │
│                                                       │
│  POST /generar-prueba        → Circom + snarkjs       │
│  POST /casper/registrar-evento → Casper Testnet       │
│  GET  /casper/balance          → Saldo CSPR           │
└──────────────────────────────────────────────────────┘
                      ↕
┌──────────────────────────────────────────────────────┐
│          AnitraxRegistry Contract (Odra/Rust)         │
│               Casper Testnet                          │
│                                                       │
│  register_event(id, type, sha256_hash, timestamp)     │
│  get_event_count() → u64                             │
└──────────────────────────────────────────────────────┘
```

♿ Accesibilidad — 3 modos para todos

| Modo | Para quién | Cómo funciona |
|---|---|---|
| **Estándar** | Uso general | Interfaz visual + selector multi-red |
| **Sinestético** | Discapacidad visual | 100% por voz — el agente guía cada paso |
| **Visual** | Discapacidad auditiva | Sin audio, botones grandes, máximo contraste |

Los 3 modos incluyen 5 filtros de daltonismo. Porque el acceso al dinero no debería depender de tus capacidades.

 🔐 Seguridad multicapa

| Tecnología | Función |
|---|---|
| BiometricPrompt | Autenticación por huella digital |
| Voice Authentication | Firma de voz única del usuario |
| ZK Proofs (Circom) | Privacidad criptográfica de fondos |
| Bluetooth Encryption | Protección de datos P2P |
| Ethereum Sepolia | Registro permanente de transacciones |
| **Casper Testnet** | **Auditoría inmutable del agente IA** |

---

 📈 DeFi Simulation Engine

Sistema de rendimientos por niveles basado en volumen mensual de transferencias:

| Tier | Volumen | Rendimiento |
|---|---|---|
| 🥉 Starter | 0 – 0.1 ETH/mes | 2% mensual |
| 🥈 Explorer | 0.1 – 0.5 ETH/mes | 3% mensual |
| 🥇 Builder | 0.5 – 2 ETH/mes | 5% mensual |
| 💎 Validator | 2 – 5 ETH/mes | 6% mensual |
| 🐋 Whale | 5+ ETH/mes | 8% mensual |

 🚀 Instalación rápida

```bash
# 1. Clonar
git clone https://github.com/franciscoalmarazlap-cmd/anitrax-zk.github.com

# 2. Levantar servidor (ZK + Casper)
py -m pip install fastapi uvicorn pycspr
py -m uvicorn anitrax_server:app --host 0.0.0.0 --port 8001 --reload

# 3. Exponer con ngrok
ngrok http 8001

# 4. Abrir en Android Studio → Run ▶
```

**Requisitos:** Android 8.0+, Bluetooth, Python 3.10+, Node.js, MetaMask

 🧪 Prueba en 60 segundos

1. Abre ANITRAX en **2 celulares** con Bluetooth activado
2. Selecciona un contacto → ingresa monto (ej. `0.1`)
3. Toca **Enviar** → elige wallet: **Casper Testnet**
4. Confirma por voz o huella
5. Observa en el servidor: `POST /casper/registrar-evento 200 OK`
6. Verifica el evento en [testnet.cspr.live](https://testnet.cspr.live/account/02021ece4a3a164adff5b5a1a6614c451dd562783e6ede022cd79d088a63cd66d37f)

 ⚠️ Notas

- Redes de **prueba únicamente** — no usar con tokens reales
- El servidor ZK + Casper debe estar corriendo durante las pruebas
- La URL ngrok puede cambiar al reiniciar — actualizar `BASE_URL` en `ZkProofClient.kt` y `CasperManager.kt`

 📄 Licencia

MIT License solo para participar en el hacketon — Copyright (c) 2026 Francisco Almaraz Ocelo


ANITRAX es una aplicación funcional desarrollada para el Casper Agentic Buildathon 2026.
