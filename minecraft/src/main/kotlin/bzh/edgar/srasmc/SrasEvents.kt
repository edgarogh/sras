package bzh.edgar.srasmc

import org.bukkit.*
import org.bukkit.block.Barrel
import org.bukkit.block.BlockFace
import org.bukkit.block.data.type.Chest
import org.bukkit.block.data.type.Dispenser
import org.bukkit.event.EventHandler
import org.bukkit.event.Listener
import org.bukkit.event.block.*
import org.bukkit.event.player.PlayerInteractEvent
import org.bukkit.inventory.ItemStack
import java.util.*
import kotlin.math.min

class SrasEvents(private val plugin: SrasMinecraft) : Listener {
    private val random = Random()
    private var watering: Location? = null
    private var remaining: Int = 0

    fun tick() {
        watering?.run {
            if (remaining > 0) {
                remaining--;
                world?.spawnParticle(Particle.FALLING_WATER, this, min(5, remaining / 2))
            }
        }

        plugin.plant?.run {
            if ((world?.time ?: 1) % 40 == 0L) {
                val loc = Location(world, x, y, z);
                loc.add(random.nextDouble(), random.nextDouble(), random.nextDouble())
                val connection = plugin.connection != null
                world?.spawnParticle(Particle.REDSTONE, loc, 1, Particle.DustOptions(if (connection) Color.RED else Color.fromRGB(0x770000), 1.0F))
            }
        }

        plugin.run {
            while (!queue.isEmpty()) {
                when (val msg = queue.poll()) {
                    is Connection.InMessage.Pong -> { }
                    is Connection.InMessage.Time -> {
                        plugin.plant?.world?.time = msg.value.toLong()
                    }
                    is Connection.InMessage.Humi -> {
                        plugin.plant?.block?.getRelative(BlockFace.DOWN)?.let { blockUnder ->
                            if (blockUnder.type == Material.BARREL) {
                                val barrel = blockUnder.state as Barrel
                                val itemsToAdd = msg.value * barrel.inventory.size * 64 / 1024
                                barrel.inventory.clear()
                                barrel.inventory.addItem(ItemStack(Material.OAK_SAPLING, itemsToAdd))
                            }
                        }
                    }
                }

            }
        }
    }

    @EventHandler
    fun interactEvent(e: PlayerInteractEvent) {
        val clickedBlock = e.clickedBlock

        if (plugin.plant != null && e.action == Action.RIGHT_CLICK_BLOCK && e.player.inventory.itemInMainHand.type == Material.POTION && plugin.isPlant(clickedBlock!!)) {
            e.isCancelled = true
            if (remaining > 0) {
                e.player.world.playSound(e.player.location, Sound.BLOCK_GLASS_STEP, 1F, 1F)
                return
            } else {
                e.player.world.playSound(e.player.location, Sound.ITEM_BUCKET_EMPTY, 1F, 1F)
            }

            watering = clickedBlock.location.clone().add(.5, 1.0, .5)
            remaining = 60

            plugin.connection?.dispense()
        }

        if (e.action == Action.RIGHT_CLICK_BLOCK && e.clickedBlock != null && plugin.isPlant(e.clickedBlock!!.getRelative(BlockFace.UP))) {
            e.isCancelled = true
        }
    }

    @EventHandler
    fun dispenseEvent(e: BlockDispenseEvent) {
        println("dispense")
        val blockData = e.block.blockData

        if (plugin.plant != null && e.item.type == Material.POTION && blockData is Dispenser && remaining <= 0) {
            println("dispense1")
            val facingBlock = e.block.getRelative(blockData.facing)
            println("facingBlock=$facingBlock")
            println("plant=${plugin.plant}")
            if (plugin.isPlant(facingBlock) || plugin.isPlant(facingBlock.getRelative(BlockFace.DOWN))) {
                println("dispense2")
                e.isCancelled = true
                watering = facingBlock.location.clone().add(.5, 1.0, .5)
                remaining = 60

                plugin.connection?.dispense()
            }
        }
    }

    @EventHandler
    fun breakEvent(e: BlockBreakEvent) {
        if (plugin.isPlant(e.block)) {
            plugin.plant = null
            plugin.server.broadcastMessage("La plante est cassée ;(")
        }
    }
}