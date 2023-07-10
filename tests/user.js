import { TokenHelper } from "./token_helper.js";

class User {
    usdcToken;
    usdcTokenAccount;
    vaultToken;
    vaultTokenAccount;
    wallet;

    constructor(wallet, usdcMintAddress, vaultMintAddress) {
        this.wallet = wallet;
        this.usdcToken = new TokenHelper(usdcMintAddress);
        this.vaultToken = new TokenHelper(vaultMintAddress);
    }

    getOrCreateUSDCAccount = async (connection, payer) => {
        this.usdcTokenAccount = (await this.usdcToken.getOrCreateTokenAccount(
            connection,
            payer,
            this.wallet.publicKey
        )).address
    }

    getOrCreateVaultAccount = async (connection, payer) => {
        this.vaultTokenAccount = (await this.vaultToken.getOrCreateTokenAccount(
            connection,
            payer,
            this.wallet.publicKey
        )).address
    }

    usdcBalance = async (connection) => {
        return await this.usdcToken.balance(connection, this.usdcTokenAccount)
    }

    vaultBalance = async (connection) => {
        return await this.vaultToken.balance(connection, this.vaultTokenAccount)
    }
}

export { User };