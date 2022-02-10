-- CreateTable
CREATE TABLE "profiles" (
    "id" VARCHAR(64) NOT NULL,
    "servant_id" INTEGER NOT NULL,
    "number" INTEGER NOT NULL,
    "text" TEXT NOT NULL,

    CONSTRAINT "profiles_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "profiles_servant_id_idx" ON "profiles"("servant_id");

-- AddForeignKey
ALTER TABLE "profiles" ADD CONSTRAINT "profiles_servant_id_fkey" FOREIGN KEY ("servant_id") REFERENCES "servants"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
