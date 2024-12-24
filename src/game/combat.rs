pub enum CombatAction {
    Attack,
    Eat,
    Skill,
    Retreat,

}
pub enum CombatState {
    PlayerTurn,
    NPCTurn,
    Defeat,
    Victory,
    Fled

}