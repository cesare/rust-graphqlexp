-- CreateTable
CREATE TABLE "servants" (
    "id" SERIAL NOT NULL,
    "name" VARCHAR(100) NOT NULL,
    "class_name" VARCHAR(64) NOT NULL,
    "rarity" INTEGER NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "servants_pkey" PRIMARY KEY ("id")
);
