# solana-hackathon2023

  > ## Contents


* [Features](https://github.com/MIDAV0/solana-hackathon2023#Abstract)
    - [Team Members]()
    - [Strategy](https://github.com/MIDAV0/solana-hackathon2023#Team-Members)

> ## Abstract
Bank Boss is designed at its core to be simplistic and user friendly with the capability to allow borrowers and lenders to leverage their assets
to create an easier platform to leverage assets within the crypto eco-system. Currently individuals have a lenghty process to exit the market and in many instances do not wish to
sell their positions for loss or for percieved loss. Bank Boss will allow them to lock their tokens at an agreed amount and make regular payments to recieve their 
assets back at the loan price. This can create a scenario whereby should the asset increase to a greater amount than originally agreed the 
borrower can still relaise the value which the lender recieves the interest agreed + original capital.

## Team Members
- [Vadzim Mahilny](https://github.com/MIDAV0)
- [Karl Timmins](https://github.com/Karlitoyo)

## High-level overview of the project
![Alt text](solanahackathon-Page-1.drawio.png)

## Project implementation
Tech stack:
- [Anchor](https://project-serum.github.io/anchor/getting-started/introduction.html) - Solana smart contract framework
- [React](https://reactjs.org/) - Front end framework
- [Solana](https://solana.com/) - Blockchain

 > # UX

The UX has been designed to be simple and with the aim of targeting the most financially illiterate individuals
with the purpose of easily encouraging individuals to provide liquidity and connect to the platform. 

## Users

The website has a target audience of both new entrants to banking system and existing users. The age 
profile is based on bank account users within our targeted demographic. Any individual with digital assets can make use of the service.

## Strategy

Initial strategy will be to target crypto users who currently require cash for their assests. This will allow individuals to
retain their assets at the price agreed when taking the loan and to repay the loan to redeem their asset.

## Scope

The Scope of the service is to create financial literacy and encourage saving from a wider range of individuals.
This product can benefit individuals who rely on high interest loans or payday loans, this service can also benefit
current mortgage holders who are struggling to meet regular repayments or who may be in negative equity.


## Wireframes
### Landing Page:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/8b487bfd-904d-48a1-9d59-a2cffd238760">
</p>

### Lending Page:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/90321821-cd06-4ca1-9249-66d8301d8320">
</p>

### Lending Page selected:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/c52e4af0-8dc7-4852-9643-472f7f45164e">
</p>

### Borrow Page:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/e6700b43-2bad-448e-9f74-906ed325b7bc">
</p>

### Borrow Page selected:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/e294dec8-fb13-4703-bf11-ef21a9d9a243">
</p>

> # User-Stories

## First case example user experience

Middle income user who has difficulty meeting loan obligations and struggles with regular savings -

Individual who holds crypto assets requires cash loan to make payment for holiday. User feels asset will rise in value in the short to
medium term. User can trade the asset on exhange for Ethereum or snother asset then pay high fees through crypto bank for cash
and withdraw. Making use of Bank-Boss users can trade the token they hold for cash at an agreed amount, term, and rate of interest. Withdraw
the cash to bank and repay loan over term. During the term of the loan the value of the asset held as collateral increases 2x. The user repays the
loan in full to the lender capital + interest and the asset is released at the original price at the borrower recieved the funds for. The borrower
can then sell of re-loan based on the original agreement being met.

## Second case example user experience

Low income user who is in arrears on mortgage and cannot maintain current levels of debt to salary -

Borrower visits site. Borrower has digital assets of value €1100 which places against a loan as collateral for €1000 + 10% interest. User agrees term of 11 months @ €100 per month. The borrower makes 5 payments of €100. During this period the value of the asset drops to €600, the borrower
decides not to continue with the repayment of the loan. The asset + €500 paid in the 5 monthly installments revert to the lender.

## Instruction pipeline
![Alt text](solanahackathon-Instructions.drawio.png)

## SWOT

- Strengths

Personal finances applications market is an increasing market. Recent analysis by Statista -
(https://www.statista.com/outlook/298/109/personal-finance/united-states)
indicates market growth to over €1 Billion in value by 2023 in the United States alone. A growth of 25%
from its current position.

The system has numerous strengths which include simple user-friendly UI making use of blockchain technology
, dectralized finance and allowing users to leverage their assets.

- Weaknesses (Challenges)

Challenges identified predominantly relate to smart contract development.

- Opportunities

This model is not in practice currently. Lending within the crypto eco-system is not fully adopted however this is an area within
the space which will become more prevalent as the industry matures.

- Threats

Threats relate to similar products. However this is an emerging space within the crypto industry. Threats also relate to the value of
assets reducing leaving investor open to losses, similar to impairement loss currently faced through providing liquidity. Threats also 
relate to individuals not repaying loans upon taking loan. The interest cost being fixed within the original asset collatoral can mitigate this
risk somewhat.

## How to run

`anchor build` - build smart contracts

`anchor test` - test contract

`anchor deploy` -  deploy

`yarn install`

Front end is in `frontend` folder (react-app).
