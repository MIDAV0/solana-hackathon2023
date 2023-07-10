import {
    getMint,
    getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

class TokenHelper {
    mint;

    constructor(mint) {
        this.mint = mint;
    }

    getMint = async (connection) => {
        return (await getMint(connection, this.mint)).address;
    }

    balance = async (connection, token_account) => {
        return parseInt((await connection.getTokenAccountBalance(token_account)).value.amount)
    }

    getOrCreateTokenAccount = async (connection, payer, owner, isPDA = false) => {
        return await getOrCreateAssociatedTokenAccount(
            connection,
            payer,
            this.mint,
            owner,
            isPDA
        )
    }
}

export { TokenHelper };