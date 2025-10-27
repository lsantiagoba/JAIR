# Troubleshooting - JAIR Snap

## Problemas Actuales

### 1. Botón "+" (Agregar Imágenes) No Funciona

**Para diagnosticar**, ejecuta:

```bash
# Instala el nuevo snap
./INSTALL_SNAP.sh

# Verifica las conexiones
snap connections jair

# Ejecuta con debug
GTK_DEBUG=interactive snap run jair
```

**Cuando hagas clic en el botón "+", anota:**
- ¿Aparece algún mensaje de error en la terminal?
- ¿Se abre el diálogo de archivos?
- ¿Qué pasa exactamente?

### 2. Traducciones al Español No Funcionan

**Estado**: He agregado soporte para preservar el locale del sistema.

**Para probar:**

```bash
# Reinstala el snap actualizado
./INSTALL_SNAP.sh

# Verifica que tu sistema esté en español
echo $LANG
echo $LANGUAGE

# Ejecuta JAIR
snap run jair
```

**Si aún no funciona**, prueba forzar el idioma:

```bash
LANGUAGE=es_CO snap run jair
```

O:

```bash
LANG=es_CO.UTF-8 snap run jair
```

## Diagnóstico Detallado

### Ejecuta el script de diagnóstico:

```bash
./DEBUG_SNAP.sh
```

Esto mostrará:
- Conexiones del snap
- Estado del portal de archivos
- Variables de entorno
- Archivos de traducción

### Verifica el portal de archivos:

```bash
# Debe estar corriendo
ps aux | grep xdg-desktop-portal

# Si no está corriendo, inícialo
/usr/libexec/xdg-desktop-portal &
```

### Verifica permisos:

```bash
# Debe mostrar "connected" para todos
snap connections jair | grep -E "(home|desktop|mount)"
```

## Posibles Soluciones

### Si el botón "+" no funciona:

1. **Verifica que xdg-desktop-portal esté corriendo**:
   ```bash
   systemctl --user status xdg-desktop-portal
   ```

2. **Reinicia el portal**:
   ```bash
   systemctl --user restart xdg-desktop-portal
   ```

3. **Verifica el backend del portal para tu escritorio**:
   - GNOME: `xdg-desktop-portal-gnome`
   - KDE: `xdg-desktop-portal-kde`
   - Otros: `xdg-desktop-portal-gtk`

   ```bash
   # Instala el backend si falta
   sudo apt install xdg-desktop-portal-gnome  # Para GNOME
   ```

### Si las traducciones no funcionan:

1. **Verifica que el locale español esté generado en tu sistema**:
   ```bash
   locale -a | grep es
   ```

2. **Si no aparece español, genera los locales**:
   ```bash
   sudo locale-gen es_CO.UTF-8
   sudo update-locale
   ```

3. **Configura tu sistema en español**:
   ```bash
   # Edita ~/.profile o ~/.bashrc
   export LANG=es_CO.UTF-8
   export LANGUAGE=es_CO:es
   ```

## Información para Reportar

Si los problemas persisten, reporta lo siguiente:

1. Salida de `./DEBUG_SNAP.sh`
2. Errores al hacer clic en "+": (pega la salida de `snap run jair`)
3. Tu entorno de escritorio: (GNOME, KDE, etc.)
4. Versión de Ubuntu: `lsb_release -a`
5. ¿Está instalado xdg-desktop-portal?: `dpkg -l | grep xdg-desktop-portal`
