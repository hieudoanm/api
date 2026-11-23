package com.api.idea

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.components.PersistentStateComponent
import com.intellij.openapi.components.State
import com.intellij.openapi.components.Storage

@State(name = "ApiSettings", storages = [Storage("api.xml")])
class ApiSettings : PersistentStateComponent<ApiSettings> {
    var cliPath: String = "api"

    override fun getState(): ApiSettings = this
    override fun loadState(state: ApiSettings) {
        cliPath = state.cliPath
    }

    companion object {
        val instance: ApiSettings
            get() = ApplicationManager.getApplication().getService(ApiSettings::class.java)

        var cliPath: String
            get() = instance.cliPath
            set(value) {
                instance.cliPath = value
            }
    }
}
