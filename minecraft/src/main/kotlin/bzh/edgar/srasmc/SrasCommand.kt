package bzh.edgar.srasmc

import org.bukkit.Material
import org.bukkit.command.Command
import org.bukkit.command.CommandExecutor
import org.bukkit.command.CommandSender
import org.bukkit.command.TabCompleter
import org.bukkit.entity.Player
import java.io.DataInputStream
import java.net.*
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.concurrent.thread

class SrasCommand(private val plugin: SrasMinecraft) : CommandExecutor, TabCompleter {
    override fun onCommand(sender: CommandSender, command: Command, label: String, args: Array<out String>): Boolean {
        return if (!sender.isOp) {
            sender.sendMessage("Ptdrr t ki")
            true
        } else {
            if (args.isEmpty()) {
                return false
            } else if (args[0] == "set-time") {
                plugin.queue.offer(Connection.InMessage.Time(args[1].toInt()))
                return true
            } else if (args[0] == "set-humi") {
                plugin.queue.offer(Connection.InMessage.Humi(args[1].toInt()))
                return true
            } else if (args[0] == "plant" && sender is Player) {
                val rayTrace = sender.rayTraceBlocks(10.0)

                rayTrace?.run {
                    val hitBlock = hitBlock
                    if (hitBlock != null && hitBlock.type == Material.POTTED_OAK_SAPLING) {
                        println("set plant ${hitBlock.location.x} ${hitBlock.location.y} ${hitBlock.location.z}")
                        plugin.plant = hitBlock.location
                        sender.sendMessage("Plante définie")
                    } else {
                        sender.sendMessage("C'est pas une plante")
                    }
                }

                if (rayTrace == null) {
                    sender.sendMessage("J'ai rien trouvé")
                }

                return true
            } else if (args[0] == "connect") {
                if (args.size == 2) {
                    try {
                        val uri = URI("any://" + args[1])
                        val addr = uri.host
                        val port = when (uri.port) {
                            -1 -> 1918
                            else -> uri.port
                        }

                        plugin.connection = Connection.open(plugin, sender, InetSocketAddress(addr, port))
                    } catch (e: URISyntaxException) {
                        sender.sendMessage("Endpoint invalide")
                    }
                    return true
                } else {
                    return false
                }
            } else if (args[0] == "disconnect") {
                plugin.connection?.stop()
                sender.sendMessage("Déconnecté")
                return true
            }
            false
        }
    }

    private var pinging: AtomicBoolean = AtomicBoolean(false)
    private var found: AtomicBoolean = AtomicBoolean(false)

    override fun onTabComplete(
        sender: CommandSender,
        command: Command,
        alias: String,
        args: Array<out String>
    ): MutableList<String> {
        return when {
            args[0] == "connect" -> {
                if (found.get()) {
                    return mutableListOf("localhost:1918")
                } else if (!pinging.get()) {
                    pinging = AtomicBoolean(true)
                    thread {
                        try {
                            val socket = Socket(InetAddress.getLocalHost(), 1918)
                            socket.getOutputStream().write(byteArrayOf(0, 0, 0, 0))
                            found = AtomicBoolean(DataInputStream(socket.getInputStream()).readInt() == 1918)
                            socket.close()
                            println("connect attempt: success")
                        } catch (e: ConnectException) {
                            println("connect attempt: fail")
                        } finally {
                            pinging = AtomicBoolean(false)
                        }
                    }
                }

                mutableListOf()
            }
            args.size < 2 -> {
                mutableListOf("set-time", "set-humi", "plant", "connect", "disconnect")
            }
            else -> {
                mutableListOf()
            }
        }
    }

}