use crate::analisador_lexico::Simbolo;
use crate::arvore_sintatica_abstrata::Expr;

pub struct Analisador {
    simbolos: Vec<Simbolo>,
    atual: usize,
}

impl Analisador {
    pub fn new(simbolos: Vec<Simbolo>) -> Self {
        Analisador { simbolos, atual: 0 }
    }

    fn avancar(&mut self) {
        if self.atual < self.simbolos.len() {
            self.atual += 1;
        }
    }

    fn ver(&self) -> Option<&Simbolo> {
        self.simbolos.get(self.atual)
    }

    fn consumir(&mut self, esperado: &Simbolo) -> bool {
        if let Some(atual) = self.ver() {
            if atual == esperado {
                self.avancar();
                return true;
            }
        }
        false
    }

    pub fn analisar_expressao(&mut self) -> Option<Expr> {
        // Primeiro verifica se é um bloco
        if let Some(Simbolo::AbreChaves) = self.ver() {
            self.analisar_bloco()
        } else if let Some(Simbolo::If) = self.ver() {
            self.analisar_if()
        } else if let Some(Simbolo::While) = self.ver() {
            self.analisar_while()
        } else if let Some(Simbolo::Var) = self.ver() {
            self.analisar_declaracao_variavel()
        } else {
            self.analisar_atribuicao()
        }
    }

    fn analisar_while(&mut self) -> Option<Expr> {
        // while ( expression ) statement
        if !self.consumir(&Simbolo::While) {
            return None;
        }

        // Consome o parêntese de abertura
        if !self.consumir(&Simbolo::AbreParenteses) {
            return None;
        }

        // Analisa a condição
        let condicao = self.analisar_atribuicao()?;

        // Consome o parêntese de fechamento
        if !self.consumir(&Simbolo::FechaParenteses) {
            return None;
        }

        // Analisa o corpo do while
        let corpo = self.analisar_declaracao()?;

        Some(Expr::While(Box::new(condicao), Box::new(corpo)))
    }

    fn analisar_if(&mut self) -> Option<Expr> {
        // if ( expression ) statement else statement
        if !self.consumir(&Simbolo::If) {
            return None;
        }

        // Consome o parêntese de abertura
        if !self.consumir(&Simbolo::AbreParenteses) {
            return None;
        }

        // Analisa a condição
        let condicao = self.analisar_atribuicao()?;

        // Consome o parêntese de fechamento
        if !self.consumir(&Simbolo::FechaParenteses) {
            return None;
        }

        // Analisa o ramo then
        let then_expr = self.analisar_declaracao()?;

        // Verifica se há um ramo else
        let else_expr = if let Some(Simbolo::Else) = self.ver() {
            self.avancar(); // consome o else
            Some(Box::new(self.analisar_declaracao()?))
        } else {
            None
        };

        Some(Expr::If(Box::new(condicao), Box::new(then_expr), else_expr))
    }

    fn analisar_bloco(&mut self) -> Option<Expr> {
        // { declaration* }
        if !self.consumir(&Simbolo::AbreChaves) {
            return None;
        }

        let mut declaracoes = Vec::new();

        // Analisa declarações até encontrar }
        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::FechaChaves => {
                    self.avancar(); // consome o }
                    break;
                }
                Simbolo::PontoVirgula => {
                    self.avancar(); // consome o ;
                    continue; // pula para a próxima declaração
                }
                _ => {
                    // Tenta analisar uma declaração
                    if let Some(declaracao) = self.analisar_declaracao() {
                        declaracoes.push(declaracao);
                    } else {
                        return None; // erro na declaração
                    }
                }
            }
        }

        Some(Expr::Block(declaracoes))
    }

    fn analisar_declaracao(&mut self) -> Option<Expr> {
        // Uma declaração pode ser uma declaração de variável, um bloco ou uma expressão
        match self.ver() {
            Some(Simbolo::Var) => self.analisar_declaracao_variavel(),
            Some(Simbolo::AbreChaves) => self.analisar_bloco(),
            _ => self.analisar_atribuicao(),
        }
    }

    fn analisar_declaracao_variavel(&mut self) -> Option<Expr> {
        // var IDENTIFIER "=" assignment | var IDENTIFIER
        if !self.consumir(&Simbolo::Var) {
            return None;
        }

        // Consome o identificador
        let nome = if let Some(Simbolo::Identificador(nome)) = self.ver() {
            let nome = nome.clone();
            self.avancar();
            nome
        } else {
            return None;
        };

        // Verifica se há inicialização
        if let Some(Simbolo::Igual) = self.ver() {
            self.avancar(); // consome o =
            let valor = self.analisar_atribuicao()?;
            Some(Expr::VarDef(nome, Box::new(valor)))
        } else {
            // Se não há inicialização, trata como var variavel = nil
            Some(Expr::VarDef(nome, Box::new(Expr::Numero(0.0)))) // nil = 0.0 para simplificar
        }
    }

    fn analisar_atribuicao(&mut self) -> Option<Expr> {
        // assignment  → IDENTIFIER '=' assignment | logic_or ;
        let expr = self.analisar_or()?;
        if let Some(Simbolo::Igual) = self.ver() {
            self.avancar();
            if let Expr::Identificador(nome) = expr {
                let valor = self.analisar_atribuicao()?;
                return Some(Expr::Assign(nome, Box::new(valor)));
            } else {
                // Erro: lado esquerdo não é identificador
                return None;
            }
        }
        Some(expr)
    }

    fn analisar_or(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_and()?;

        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::Or => {
                    self.avancar();
                    let direito = self.analisar_and()?;
                    expr = Expr::Or(Box::new(expr), Box::new(direito));
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn analisar_and(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_comparacao()?;

        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::And => {
                    self.avancar();
                    let direito = self.analisar_comparacao()?;
                    expr = Expr::And(Box::new(expr), Box::new(direito));
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn analisar_soma(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_produto()?;

        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::Soma => {
                    self.avancar();
                    let direito = self.analisar_produto()?;
                    expr = Expr::Soma(Box::new(expr), Box::new(direito));
                }
                Simbolo::Subtracao => {
                    self.avancar();
                    let direito = self.analisar_produto()?;
                    expr = Expr::Subtracao(Box::new(expr), Box::new(direito));
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn analisar_comparacao(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_soma()?;

        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::Maior => {
                    self.avancar();
                    let direito = self.analisar_soma()?;
                    expr = Expr::Maior(Box::new(expr), Box::new(direito));
                }
                Simbolo::MaiorIgual => {
                    self.avancar();
                    let direito = self.analisar_soma()?;
                    expr = Expr::MaiorIgual(Box::new(expr), Box::new(direito));
                }
                Simbolo::Menor => {
                    self.avancar();
                    let direito = self.analisar_soma()?;
                    expr = Expr::Menor(Box::new(expr), Box::new(direito));
                }
                Simbolo::MenorIgual => {
                    self.avancar();
                    let direito = self.analisar_soma()?;
                    expr = Expr::MenorIgual(Box::new(expr), Box::new(direito));
                }
                Simbolo::IgualIgual => {
                    self.avancar();
                    let direito = self.analisar_soma()?;
                    expr = Expr::IgualIgual(Box::new(expr), Box::new(direito));
                }
                Simbolo::Diferente => {
                    self.avancar();
                    let direito = self.analisar_soma()?;
                    expr = Expr::Diferente(Box::new(expr), Box::new(direito));
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn analisar_produto(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_atributo()?;

        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::Multiplicacao => {
                    self.avancar();
                    let direito = self.analisar_atributo()?;
                    expr = Expr::Multiplicacao(Box::new(expr), Box::new(direito));
                }
                Simbolo::Divisao => {
                    self.avancar();
                    let direito = self.analisar_atributo()?;
                    expr = Expr::Divisao(Box::new(expr), Box::new(direito));
                }
                _ => break,
            }
        }

        Some(expr)
    }

    fn analisar_unario(&mut self) -> Option<Expr> {
        match self.ver()? {
            Simbolo::Negacao => {
                self.avancar(); // consome o !
                let expr = self.analisar_unario()?;
                Some(Expr::Negacao(Box::new(expr)))
            }
            Simbolo::Subtracao => {
                self.avancar(); // consome o -
                let expr = self.analisar_unario()?;
                Some(Expr::NegacaoAritmetica(Box::new(expr)))
            }
            Simbolo::Numero(n) => {
                let valor = *n;
                self.avancar();
                Some(Expr::Numero(valor))
            }
            Simbolo::String(s) => {
                let valor = s.clone();
                self.avancar();
                Some(Expr::String(valor))
            }
            Simbolo::Identificador(id) => {
                let nome = id.clone();
                self.avancar();
                Some(Expr::Identificador(nome))
            }
            Simbolo::AbreParenteses => {
                self.avancar();
                let expr = self.analisar_expressao()?;
                if self.consumir(&Simbolo::FechaParenteses) {
                    Some(expr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn analisar_atributo(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_chamada()?;
        
        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::Ponto => {
                    self.avancar(); // consome o ponto
                    if let Some(Simbolo::Identificador(attr)) = self.ver() {
                        let nome_attr = attr.clone();
                        self.avancar(); // consome o identificador
                        expr = Expr::GetAttr(Box::new(expr), nome_attr);
                    } else {
                        return None; // esperava um identificador após o ponto
                    }
                }
                _ => break,
            }
        }
        
        Some(expr)
    }

    fn analisar_chamada(&mut self) -> Option<Expr> {
        let mut expr = self.analisar_unario()?;
        
        while let Some(simbolo) = self.ver() {
            match simbolo {
                Simbolo::AbreParenteses => {
                    self.avancar(); // consome o parêntese de abertura
                    let mut argumentos = Vec::new();
                    
                    // Verifica se há argumentos
                    if let Some(simbolo) = self.ver() {
                        if *simbolo != Simbolo::FechaParenteses {
                            // Primeiro argumento
                            if let Some(arg) = self.analisar_expressao() {
                                argumentos.push(arg);
                            }
                            
                            // Argumentos adicionais separados por vírgula
                            while let Some(simbolo) = self.ver() {
                                match simbolo {
                                    Simbolo::Virgula => {
                                        self.avancar(); // consome a vírgula
                                        if let Some(arg) = self.analisar_expressao() {
                                            argumentos.push(arg);
                                        } else {
                                            return None; // esperava um argumento após a vírgula
                                        }
                                    }
                                    Simbolo::FechaParenteses => break,
                                    _ => return None, // caractere inesperado
                                }
                            }
                        }
                    }
                    
                    // Consome o parêntese de fechamento
                    if self.consumir(&Simbolo::FechaParenteses) {
                        expr = Expr::Call(Box::new(expr), argumentos);
                    } else {
                        return None; // esperava parêntese de fechamento
                    }
                }
                _ => break,
            }
        }
        
        Some(expr)
    }
}