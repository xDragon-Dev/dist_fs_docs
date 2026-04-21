/*
TODO: TESTING 🤜
*PASSED ✅👍
! FAILED ❌👎

El administrador podrá realizar las siguientes operaciones: 
- Dar de alta un usuario
- Dar de baja un usuario
- Eliminar temática o subtemática de cualquier usuario aunque no estén vacías
- Modificar nombres de usuario y/o contraseñas.

Operaciones:
Las operaciones que permitirá son:
- Registrar usuario (no hay necesidad de que el administrador lo realice). El usuario deberá elegir un nombre de usuario y contraseña.
- Ingresar al sistema, con nombre de usuario y contraseña.
- Dar de alta una temática
- Dar de alta una subtemática
- Eliminar una temática o subtemática (siempre cuando éstas se encuentren vacías)
- Subir un documento
- Descargar un documento
- Eliminar un documento
*/

/*
LISTA DE TABLAS EXISTENTES
-usuarios
-documentos
-temas
-subtemas
-temas_por_documento
-subtemas_por_documento
-nodos_de_metadatos
-nodos_de_almacenamiento
-nodos_de_almacenamiento_por_documento
*/

/*
TODO: Altas, Bajas, Modificaciones
ENLISTADO DE TODAS LAS CONSULTAS

*CONSULTAS DE USUARIO
-Dar de alta usuario
-Dar de alta usuario admin
-Dar de baja usuario
-Actualizar rol
-----------------------------------------------------

*CONSULTAS DE DOCUMENTO
-Dar de alta un docuento ✅
-Dar de baja documento
-Obtener datos de un documento ✅
-----------------------------------------------------
Obtener todos los documentos relacionados a un usuario
?Obtener los documentos relacionados a un topico
?Obtener documentos relacionados a una subtopico (o más subtopicos, no sé cómo hacerlo)
!Obtener documentos basados en un metadato (titulo autores, no sé si se pueda hacer eficientemente o requiero una consulta para cada cosa)

*CONSULTAS DE TOPICO
-Dar de alta tematica
-Dar de baja tematica (Sin referencias para User, con o sin referencias para Admin)
-----------------------------------------------------
Obtener todos los tópicos
?Obtener todos los topicos relacionados a un documento

*CONSULTAS DE SUBTOPICO 
-Dar de alta subtematica
-Dar de baja subtematica (Sin referencias para User, con o sin referencias para Admin)
-----------------------------------------------------
Obtener todos los subtopicos
?Obtener todos los topicos relacionados a un documento
?Obtener todos los subtopicos relacionados a un topico

*CONSULTAS DE DATANOES
-Dar de alta data nodes
-Actualizar heartbeat


*CONSULTAS DE DATANOES
-Dar de alta storage nodos
-Actualizar heartbeat
-Obtener nodos vivos
?Obtener noos relacionados a documento

*/