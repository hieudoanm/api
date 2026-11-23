package com.api.idea

import com.intellij.openapi.options.Configurable
import com.intellij.openapi.options.ConfigurationException
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.ui.components.JBLabel
import com.intellij.util.ui.FormBuilder
import java.awt.BorderLayout
import javax.swing.JComponent
import javax.swing.JPanel

class ApiSettingsConfigurable : Configurable {
    private val cliPathField = TextFieldWithBrowseButton().apply {
        text = ApiSettings.cliPath
    }
    private var panel: JPanel? = null

    override fun getDisplayName() = "api"

    override fun createComponent(): JComponent {
        val form = FormBuilder.createFormBuilder()
            .addLabeledComponent(JBLabel("api CLI path:"), cliPathField)
            .addComponentFillVertically(JPanel(), 0)
            .panel
        panel = JPanel(BorderLayout()).apply { add(form, BorderLayout.NORTH) }
        return panel!!
    }

    override fun isModified(): Boolean = cliPathField.text != ApiSettings.cliPath

    @Throws(ConfigurationException::class)
    override fun apply() {
        ApiSettings.cliPath = cliPathField.text.trim().ifEmpty { "api" }
    }

    override fun reset() {
        cliPathField.text = ApiSettings.cliPath
    }

    override fun disposeUIResources() {
        panel = null
    }
}
