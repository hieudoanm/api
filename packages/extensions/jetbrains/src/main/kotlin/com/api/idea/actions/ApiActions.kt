package com.api.idea.actions

import com.api.idea.ApiCli
import com.api.idea.ApiSettings
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.fileChooser.FileChooserDescriptor
import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.fileChooser.FileChooserFactory
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile

private fun notify(
    project: Project,
    message: String,
    type: NotificationType = NotificationType.INFORMATION,
) {
    ApplicationManager.getApplication().invokeLater {
        NotificationGroupManager.getInstance().getNotificationGroup("api")
            ?.createNotification(message, type)
            ?.notify(project)
    }
}

private fun notifyError(project: Project, message: String) =
    notify(project, message, NotificationType.ERROR)

private fun notifyInfo(project: Project, message: String) =
    notify(project, message, NotificationType.INFORMATION)

/** Runs an `api` CLI command off the EDT and reports the outcome. */
private fun runCli(
    project: Project,
    args: List<String>,
    onSuccess: ((String) -> Unit)? = null,
) {
    val cmd = listOf(ApiSettings.cliPath) + args
    ApplicationManager.getApplication().executeOnPooledThread {
        try {
            val pb = ProcessBuilder(cmd).redirectErrorStream(true)
            val proc = pb.start()
            val output = proc.inputStream.bufferedReader().readText()
            val code = proc.waitFor()
            ApplicationManager.getApplication().invokeLater {
                if (code == 0) {
                    if (onSuccess != null) onSuccess(output)
                    else notifyInfo(project, "api ${args.first()} succeeded")
                } else {
                    notifyError(
                        project,
                        "api ${args.joinToString(" ")} failed (exit $code):\n${output.take(500)}",
                    )
                }
            }
        } catch (e: Exception) {
            ApplicationManager.getApplication().invokeLater {
                notifyError(project, "Failed to run api: ${e.message}")
            }
        }
    }
}

/** The .yaml/.yml/.json file backing the current editor, if any. */
private fun currentSpecFile(e: AnActionEvent): VirtualFile? {
    val file = e.getData(CommonDataKeys.PSI_FILE)?.virtualFile
        ?: e.getData(CommonDataKeys.VIRTUAL_FILE)
        ?: return null
    return file.takeIf { it.isInLocalFileSystem && ApiCli.isSpecFileName(it.name) }
}

/** Current spec file, or a file-chooser prompt when invoked without context. */
private fun pickSpec(project: Project, e: AnActionEvent): VirtualFile? {
    currentSpecFile(e)?.let { return it }
    val descriptor = FileChooserDescriptorFactory
        .createSingleFileDescriptor()
        .withFileFilter { file -> ApiCli.isSpecFileName(file.name) }
        .withTitle("Select an OpenAPI spec")
    return FileChooserFactory.getInstance()
        .createFileChooser(descriptor, project, null)
        .choose(project)
        .firstOrNull()
}

private fun pickFormat(project: Project, formats: List<String>): String? {
    val options = formats.toTypedArray()
    val choice = com.intellij.openapi.ui.Messages.showDialog(
        project,
        "Select the target format",
        "api",
        options,
        0,
        com.intellij.openapi.ui.Messages.getQuestionIcon(),
    )
    return options.getOrNull(choice)
}

abstract class ApiSpecAction(
    text: String,
    description: String,
    private val subdir: String,
    private val formats: List<String>,
) : AnAction(text, description, null) {

    abstract fun argsFor(specPath: String, outDir: String, format: String): List<String>

    override fun update(e: AnActionEvent) {
        e.presentation.isEnabledAndVisible = e.project != null
    }

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val spec = pickSpec(project, e) ?: return
        val format = pickFormat(project, formats) ?: return
        val outDir = ApiCli.outputDirFor(spec.path, subdir)
        runCli(project, argsFor(spec.path, outDir, format)) { output ->
            notifyInfo(project, output.lineSequence().lastOrNull() ?: "api finished")
        }
    }
}

class ConvertSpecAction : ApiSpecAction(
    "Convert OpenAPI Spec",
    "Convert an OpenAPI spec into API client collections",
    "api-collections",
    ApiCli.CLIENT_FORMATS,
) {
    override fun argsFor(specPath: String, outDir: String, format: String) =
        ApiCli.convertArgs(specPath, outDir, format)
}

class PreviewSpecAction : ApiSpecAction(
    "Preview OpenAPI Spec",
    "Generate browsable HTML API docs from an OpenAPI spec",
    "api-docs",
    ApiCli.PREVIEW_FORMATS,
) {
    override fun argsFor(specPath: String, outDir: String, format: String) =
        ApiCli.previewArgs(specPath, outDir, format)
}

/** Lists the client formats supported by the installed `api` CLI. */
class ListFormatsAction : AnAction(
    "List api Formats",
    "Show the API client formats supported by the api CLI",
    null,
) {
    override fun update(e: AnActionEvent) {
        e.presentation.isEnabledAndVisible = e.project != null
    }

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        runCli(project, listOf("formats")) { output ->
            notifyInfo(project, "api formats:\n${output.trim()}")
        }
    }
}
