package com.api.idea

/** Pure helpers shared by the plugin actions — kept IDE-free so they can be
 * unit-tested without the IntelliJ test framework. */
object ApiCli {
    val CLIENT_FORMATS = listOf(
        "postman",
        "bruno",
        "insomnia",
        "hoppscotch",
        "rest-client",
        "thunder-client",
        "yaak",
        "hurl",
        "kreya",
        "curl",
        "all",
    )

    val PREVIEW_FORMATS = listOf(
        "swagger-ui",
        "redoc",
        "scalar",
        "stoplight",
        "rapidoc",
        "all",
    )

    private val SPEC_EXTENSIONS = setOf("yaml", "yml", "json")

    fun isSpecFileName(name: String): Boolean =
        name.substringAfterLast('.', "").lowercase() in SPEC_EXTENSIONS

    /** `api convert <spec> -o <outDir> -f <format>` */
    fun convertArgs(specPath: String, outDir: String, format: String): List<String> =
        listOf("convert", specPath, "-o", outDir, "-f", format)

    /** `api preview <spec> -o <outDir> -f <format>` */
    fun previewArgs(specPath: String, outDir: String, format: String): List<String> =
        listOf("preview", specPath, "-o", outDir, "-f", format)

    /** Output directory for a conversion, next to the spec file. */
    fun outputDirFor(specPath: String, subdir: String): String {
        val parent = java.io.File(specPath).parentFile
        return if (parent != null) {
            java.io.File(parent, subdir).absolutePath
        } else {
            java.io.File(subdir).absolutePath
        }
    }
}
