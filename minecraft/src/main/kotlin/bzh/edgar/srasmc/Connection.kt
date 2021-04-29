package bzh.edgar.srasmc

import org.bukkit.command.CommandSender
import java.io.DataInputStream
import java.io.DataOutputStream
import java.net.InetSocketAddress
import java.net.Socket
import java.net.SocketException
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.Executors
import kotlin.concurrent.thread

class Connection(private val queue: ConcurrentLinkedQueue<InMessage>, private val socket: Socket) {
    private val executor = Executors.newSingleThreadExecutor()
    private val sIn = DataInputStream(socket.getInputStream());
    private val sOut = DataOutputStream(socket.getOutputStream());

    private val loop = thread {
        while (!socket.isClosed) {
            when (val id = sIn.readInt()) {
                0 -> {
                    if (sIn.readInt() == 42) {
                        queue.offer(InMessage.Pong)
                    } else {
                        error("invalid pong magic")
                    }
                }
                1 -> queue.offer(InMessage.Time(sIn.readInt()))
                2 -> queue.offer(InMessage.Humi(sIn.readInt()))
                else -> error("invalid packet id: $id")
            }
        }
    }

    fun dispense() {
        executor.submit {
            synchronized(socket) {
                sOut.writeInt(1)
                sOut.writeInt(3000)
                sOut.flush()
            }
        }
    }

    fun stop() {
        synchronized(socket) {
            loop.interrupt()
            executor.shutdown()
            socket.close()
        }
    }

    fun finalize() {
        if (!socket.isClosed) {
            error("Dropping open socket")
        }
    }

    companion object {
        fun open(plugin: SrasMinecraft, initiator: CommandSender, address: InetSocketAddress): Connection? {
            return try {
                val socket = Socket()
                socket.connect(address)
                initiator.sendMessage("ok");
                Connection(plugin.queue, socket)
            } catch (e: SocketException) {
                initiator.sendMessage("error, see console")
                e.printStackTrace()
                null
            }
        }
    }

    sealed class InMessage {
        data class Time(val value: Int) : InMessage()
        data class Humi(val value: Int) : InMessage()
        object Pong : InMessage()
    }
}