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

## Subir los cambios a un repositorio remoto
~~~sh
git push origin master
~~~

# Deshacer cambios
## Deshacer cambios desde el Repository

Para traer una rama.
~~~sh
git checkout master
~~~

Para traer un archivo o grupo de archivos especifico.
~~~sh
git checkout -- {Archivo} # Si se pone un . se traen todos los archivos
~~~


Para traer de una revision concreta.
~~~sh
git checkout {Indentificador de la revision} {Archivo} # Si se pone un . se traen todos los archivos. Si en la se donde la revision se agrega un "~" y un numero n traera la revision n anterior a la indicada (tambien acepta hashes)
~~~

## Deshacer cambios desde el Staging Area

~~~sh
git reset HEAD {Archivo}
~~~

## Deshacer cambios desde el Repository y borrar el Staging Area

Este comando borra el Working directory y el Staging Area y trae una revision, este es un comando peligroso.
~~~sh
git reset --hard {Revision}
~~~

## revertir los cambios realizados de manera segura

Con el siguiente comando se puede revertir los cambio y se puede seleccionar que archivos van a ser revertidos

~~~sh
git revert {Nombre de la revision}
~~~
<<<<<<< HEAD

# Rasmas o branch

Las ramas valen para poder hacer un desarrollo en paralelo al desarrollo original o hacer pruebas sin alterar el proyecto principal.

## Crear rama a partir  de la amster

~~~sh
git branch {Nombre de la rama} master
~~~

## Pasar a la nueva rama

~~~sh
git checkout {nombre de la rama}
~~~

## Saber las ramas disponibles y en la que estamos

~~~sh
git branch # con -a podemos ver las ramas remotas y con -v sale el mensaje del ultimo commit de cada rama
~~~

## Fusionar con la rama pricipal
Primero hay que cambiar a la rama pricipal
~~~sh
git checkout master
~~~

Y fusionamos las 2 ramas
~~~sh
git merge {nombre de la rama} master
~~~
