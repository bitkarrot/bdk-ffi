package org.bitcoindevkit

import org.junit.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.assertTrue

@RunWith(AndroidJUnit4::class)
class LiveWalletTest {
    @Test
    fun testSyncedBalance() {
        val descriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet: Wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
        val esploraClient: EsploraClient = EsploraClient("https://mempool.space/testnet/api")
        // val esploraClient = EsploraClient("https://blockstream.info/testnet/api")
        val update = esploraClient.scan(wallet, 10uL, 1uL)
        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total()}")
        val balance: Balance = wallet.getBalance()
        println("Balance: $balance")

        assert(wallet.getBalance().total() > 0uL)
    }

    @Test
    fun testBroadcastTransaction() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
        val esploraClient = EsploraClient("https://mempool.space/testnet/api")
        val update = esploraClient.scan(wallet, 10uL, 1uL)

        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total()}")
        println("New address: ${wallet.getAddress(AddressIndex.New).address}")

        assert(wallet.getBalance().total() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.getAddress(AddressIndex.New).address} and try again."
        }

        val recipient: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.TESTNET)

        val psbt: PartiallySignedTransaction = TxBuilder()
            .addRecipient(recipient.scriptPubkey(), 4200uL)
            .feeRate(4.0f)
            .finish(wallet)

        println(psbt.serialize())
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")

        val walletDidSign = wallet.sign(psbt)
        assertTrue(walletDidSign)

        val tx: Transaction = psbt.extractTx()

        println("Txid is: ${tx.txid()}")
        esploraClient.broadcast(tx)
    }
}