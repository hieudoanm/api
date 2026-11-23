package com.api.idea

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ApiCliTest {
    @Test
    fun `client formats cover all ten clients plus all`() {
        assertEquals(11, ApiCli.CLIENT_FORMATS.size)
        assertTrue(ApiCli.CLIENT_FORMATS.containsAll(listOf("postman", "curl", "all")))
    }

    @Test
    fun `preview formats cover all five docs UIs plus all`() {
        assertEquals(6, ApiCli.PREVIEW_FORMATS.size)
        assertTrue(ApiCli.PREVIEW_FORMATS.containsAll(listOf("swagger-ui", "rapidoc", "all")))
    }

    @Test
    fun `isSpecFileName accepts yaml yml and json`() {
        assertTrue(ApiCli.isSpecFileName("openapi.yaml"))
        assertTrue(ApiCli.isSpecFileName("openapi.yml"))
        assertTrue(ApiCli.isSpecFileName("spec.JSON"))
        assertFalse(ApiCli.isSpecFileName("spec.txt"))
        assertFalse(ApiCli.isSpecFileName("noext"))
    }

    @Test
    fun `convertArgs builds the api convert command line`() {
        assertEquals(
            listOf("convert", "/s/spec.yaml", "-o", "/s/api-collections", "-f", "postman"),
            ApiCli.convertArgs("/s/spec.yaml", "/s/api-collections", "postman"),
        )
    }

    @Test
    fun `previewArgs builds the api preview command line`() {
        assertEquals(
            listOf("preview", "/s/spec.json", "-o", "/s/api-docs", "-f", "redoc"),
            ApiCli.previewArgs("/s/spec.json", "/s/api-docs", "redoc"),
        )
    }

    @Test
    fun `outputDirFor places the output dir next to the spec`() {
        val dir = ApiCli.outputDirFor("/project/apis/petstore.yaml", "api-collections")
        assertTrue(dir.endsWith("api-collections"))
        assertTrue(dir.startsWith("/project/apis"))
    }

    @Test
    fun `outputDirFor falls back to cwd for a bare filename`() {
        val dir = ApiCli.outputDirFor("petstore.yaml", "api-collections")
        assertTrue(dir.endsWith("api-collections"))
    }
}
