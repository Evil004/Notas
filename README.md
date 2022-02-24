# Inicio
## Basic
Git tiene 3 secciones principales Working directory, Staging Area y Repository

Working directory es donde se esta trabajando con el archivo.

Staging area son los archivos esperando a ser confirmados y subidos al repositorio

Repository es donde se almacena la version definitiva.

El archivo .gitignore permite agregar mascaras de archivos que no se tendran en cuenta.

## Iniciar GIT en un directorio
~~~sh
git init
~~~

## Pasar un archivo de Working directory a Staging Area.

~~~sh
git add {Archivo} #Si se pone un . se subiran todos los archivos del directorio.
~~~

## Obtener el estado del directorio.

Permite ver los archivos que estan o no en el Staging area, los archivos modificado, etc.
~~~sh
git status
~~~

## Pasar un archivo de Staging Area al Repository

~~~sh
git commit # Si se usa el parametro -m se puede añadir un comentario.
~~~

## Obtener datos de commits anteriores
Ver los commits pasados con su hash ID y su comentario.
~~~sh
git log 
~~~

El parametro --pretty nos permite mostrar los campos con un formato concreto.
~~~sh
git log --pretty=format:"%h %an %ar - %s" # Devuelve el hash ID, el autor, Cuanto hace que se realizo el commit y el mensaje del commit
~~~

PAra ver los canvios que hubo en un comit se usa
~~~sh
git show {hassh ID} 
~~~

## Comparar los archivos
Devuelve la diferencia de los archivos del Working directory con los del repositorio
~~~sh
git diff 
~~~
Devuelve la diferencia de los archivos del Staging Area con los del repositorio
~~~sh
git diff --staged
~~~

Devuelve la diferencia de los archivos del Working directory con los del repositorio con una herramienta grafica.

~~~sh
git difftool
~~~

# Repositorios remotos

Para conectarse a un repositorio
~~~sh
git remote add origin {Enlace} #origin es esl nombre al que nos referiremos al repositorio en el futuro, es mejor dejarlo en origin
~~~

Para ver lo que hay en el repositorio (ramas, revisiones, etc.)
~~~sh
git fetch origin 
~~~

Para descargar el repositorio
~~~sh
git pull origin master #Origin es el nombre que pusimos en el primer comando, y master es el nombre de la rama en la que lo pondremos. 
~~~

## Git Clon
Todo lo anterior se puede resumir en un comando

~~~sh
git clone {Enlace} # Si se pone un . se descarga en la carpeta actual
~~~

Subir los cambios a un repositorio remoto
~~~sh
git push origin master
~~~