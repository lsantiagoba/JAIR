#!/bin/bash

# DEBUG_SNAP.sh - Script para diagnosticar problemas del snap

echo "======================================"
echo "  JAIR - Diagnóstico del Snap"
echo "======================================"
echo ""

echo "1. Verificando conexiones del snap:"
snap connections jair
echo ""

echo "2. Verificando xdg-desktop-portal:"
ps aux | grep xdg-desktop-portal | grep -v grep
echo ""

echo "3. Verificando variables de entorno del snap:"
snap run --shell jair -c 'env | grep -E "(LANG|LANGUAGE|LC_|XDG_|SNAP)" | sort'
echo ""

echo "4. Verificando archivos de traducción:"
snap run --shell jair -c 'ls -lh $SNAP/usr/share/locale/*/LC_MESSAGES/jair.mo'
echo ""

echo "5. Verificando LANGUAGE del sistema:"
echo "LANG del sistema: $LANG"
echo "LANGUAGE del sistema: $LANGUAGE"
echo ""

echo "======================================"
echo "Ahora ejecuta: snap run jair"
echo "Y observa los errores cuando hagas clic en el botón +"
echo "======================================"
