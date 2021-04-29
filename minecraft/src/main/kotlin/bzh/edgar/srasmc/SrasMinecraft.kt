package bzh.edgar.srasmc

import org.bukkit.Location
import org.bukkit.block.Block
import org.bukkit.plugin.java.JavaPlugin
import java.io.File
import java.util.concurrent.ConcurrentLinkedQueue

class SrasMinecraft : JavaPlugin() {
    var connection: Connection? = null

    val queue = ConcurrentLinkedQueue<Connection.InMessage>()

    var plant: Location? = null

    fun isPlant(block: Block): Boolean = (
            block.world.uid == plant?.world?.uid
            && block.x == plant?.blockX
            && block.y == plant?.blockY
            && block.z == plant?.blockZ
            )

    override fun onEnable() {
        saveDefaultConfig()
        config.load(File(dataFolder, "config.yml"))
        plant = config.getLocation("plant", plant)

        val events = SrasEvents(this)
        server.scheduler.runTaskTimer(this, events::tick, 0, 1)
        server.pluginManager.registerEvents(events, this)
        server.getPluginCommand("sras")!!.run {
            val command = SrasCommand(this@SrasMinecraft)
            setExecutor(command)
            tabCompleter = command
        }
    }

    override fun onDisable() {
        config.set("plant", plant)
        config.save(File(dataFolder, "config.yml"))
        connection?.stop()
    }
}