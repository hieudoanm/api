import org.jetbrains.intellij.platform.gradle.IntelliJPlatformType

plugins {
    kotlin("jvm") version "2.4.21"
    id("org.jetbrains.intellij.platform") version "2.20.1"
    id("org.jetbrains.kotlinx.kover") version "0.9.11"
}

group = "io.github.hieudoanm.api.idea"
version = "0.0.1"

repositories {
    mavenCentral()
    intellijPlatform {
        defaultRepositories()
    }
}

kotlin {
    jvmToolchain(21)
}

dependencies {
    intellijPlatform {
        create(IntelliJPlatformType.IntellijIdeaCommunity, "2024.3")
        bundledModule("com.intellij.modules.json")
    }
    testImplementation("junit:junit:4.13.2")
}

tasks.test {
    useJUnit()
}

// Coverage is gated on the pure-Kotlin helpers (format lists, argument
// building, spec detection); IntelliJ UI/actions classes and the
// ApplicationManager-bound settings require the IDE test framework and are
// excluded from the gate.
kover {
    reports {
        filters {
            excludes {
                classes(
                    "com.api.idea.ApiSettings*",
                    "com.api.idea.actions.*",
                )
            }
        }
        verify {
            rule {
                minBound(80, coverageUnits = kotlinx.kover.gradle.plugin.dsl.CoverageUnit.LINE)
                minBound(80, coverageUnits = kotlinx.kover.gradle.plugin.dsl.CoverageUnit.BRANCH)
            }
        }
    }
}

intellijPlatform {
    pluginConfiguration {
        id = "io.github.hieudoanm.api.idea"
        name = "api — OpenAPI Converter"
        version = "0.0.1"
        description = """
            Convert OpenAPI 3.x JSON/YAML specs into API client collections
            (Postman, Bruno, Insomnia, Hoppscotch, REST Client, Thunder Client,
            Yaak, Hurl, Kreya, curl) and HTML documentation previews
            (Swagger UI, ReDoc, Scalar, Stoplight Elements, RapiDoc) via the
            `api` CLI.
        """.trimIndent()
        vendor {
            name = "hieudoanm"
        }
    }
}
