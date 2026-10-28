import csv
from collections import Counter
import matplotlib.pyplot as plt

plt.style.use('seaborn-v0_8-whitegrid' if 'seaborn-v0_8-whitegrid' in plt.style.available else 'default')
plt.rcParams['font.sans-serif'] = 'DejaVu Sans'
plt.rcParams['axes.edgecolor'] = '#cbd5e1'
plt.rcParams['axes.linewidth'] = 0.8

csv_path = 'docs/respostas_simuladas.csv'

locomocao = []
frequencia = []
problemas = []
tentou_relatar = []
aderencia = []

with open(csv_path, 'r', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    for row in reader:
        locomocao.append(row['perfil_locomocao'])
        frequencia.append(row['frequencia'])
        problemas.append(row['problema_principal'])
        problemas.append(row['problema_secundario'])
        tentou_relatar.append(row['tentou_relatar'])
        aderencia.append(row['aderencia_zelajampa'])

# 1. Gráfico Meios de Locomoção
c_loc = Counter(locomocao)
labels_loc = [
    'Automóvel\n(Aplicativo/Próprio)',
    'Motocicleta',
    'Transporte Coletivo\n(Ônibus)',
    'A pé / Bicicleta'
]
keys_map = {
    'Automóvel\n(Aplicativo/Próprio)': 'Automovel',
    'Motocicleta': 'Motocicleta',
    'Transporte Coletivo\n(Ônibus)': 'Transporte coletivo',
    'A pé / Bicicleta': 'A pe ou bicicleta'
}
counts_loc = [c_loc[keys_map[k]] for k in labels_loc]

fig, ax = plt.subplots(figsize=(7, 4), dpi=300)
colors_loc = ['#059669', '#0d9488', '#0284c7', '#64748b']
bars = ax.bar(labels_loc, counts_loc, color=colors_loc, width=0.55, edgecolor='none')

for bar in bars:
    yval = bar.get_height()
    pct = (yval / len(locomocao)) * 100
    ax.text(bar.get_x() + bar.get_width()/2, yval + 0.15, f'{yval} ({pct:.0f}%)', 
            ha='center', va='bottom', fontsize=9, fontweight='bold', color='#1e293b')

ax.set_title('1. Meio de Locomoção Predominante (N = 20)', fontsize=12, fontweight='bold', pad=15, color='#0f172a')
ax.set_ylabel('Quantidade de Respondentes', fontsize=10, color='#475569')
ax.set_ylim(0, max(counts_loc) + 1.5)
ax.grid(axis='y', linestyle='--', alpha=0.6)
ax.grid(axis='x', visible=False)
plt.tight_layout()
plt.savefig('docs/charts/01_meios_locomocao.png')
plt.close()

# 2. Gráfico Principais Problemas Enfrentados
c_prob = Counter(problemas)
prob_display = {
    'Iluminacao precaria': 'Iluminação precária / Escuridão à noite',
    'Buracos no asfalto': 'Buracos e pavimento danificado',
    'Pista estreita sem placas': 'Falta de sinalização / Estreitamento de via',
    'Calcadas quebradas': 'Calçadas e passarelas danificadas'
}
sorted_probs = sorted(c_prob.items(), key=lambda x: x[1], reverse=True)
labels_prob = [prob_display[k] for k, v in sorted_probs]
counts_prob = [v for k, v in sorted_probs]

fig, ax = plt.subplots(figsize=(8, 4), dpi=300)
colors_prob = ['#e11d48', '#ea580c', '#d97706', '#64748b']
bars = ax.barh(labels_prob[::-1], counts_prob[::-1], color=colors_prob[::-1], height=0.55)

for bar in bars:
    xval = bar.get_width()
    pct = (xval / len(locomocao)) * 100 # base respondentes
    ax.text(xval + 0.3, bar.get_y() + bar.get_height()/2, f'{xval} menções ({pct:.0f}%)', 
            ha='left', va='center', fontsize=9, fontweight='bold', color='#1e293b')

ax.set_title('2. Principais Gargalos de Risco Viário (Múltipla Escolha)', fontsize=12, fontweight='bold', pad=15, color='#0f172a')
ax.set_xlabel('Total de Menções', fontsize=10, color='#475569')
ax.set_xlim(0, max(counts_prob) + 4)
ax.grid(axis='x', linestyle='--', alpha=0.6)
ax.grid(axis='y', visible=False)
plt.tight_layout()
plt.savefig('docs/charts/02_principais_problemas.png')
plt.close()

# 3. Gráfico Hábitos de Reclamação
c_rel = Counter(tentou_relatar)
rel_labels = [
    'Não relata\n(Burocracia/Descrédito)',
    'Apenas comenta em\nWhatsApp / Redes',
    'Registrou protocolo\noficial na Prefeitura',
    'Nunca teve\nnecessidade'
]
rel_keys = {
    'Não relata\n(Burocracia/Descrédito)': 'Nao burocratico',
    'Apenas comenta em\nWhatsApp / Redes': 'Apenas WhatsApp redes',
    'Registrou protocolo\noficial na Prefeitura': 'Sim protocolo oficial',
    'Nunca teve\nnecessidade': 'Nunca necessitou'
}
counts_rel = [c_rel[rel_keys[k]] for k in rel_labels]

fig, ax = plt.subplots(figsize=(6, 4.5), dpi=300)
colors_pie = ['#ef4444', '#f59e0b', '#10b981', '#94a3b8']
wedges, texts, autotexts = ax.pie(
    counts_rel, 
    labels=rel_labels, 
    autopct='%1.0f%%',
    startangle=140,
    colors=colors_pie,
    pctdistance=0.75,
    wedgeprops=dict(width=0.45, edgecolor='white', linewidth=2)
)

for autotext in autotexts:
    autotext.set_color('white')
    autotext.set_fontweight('bold')
    autotext.set_fontsize(10)

ax.set_title('3. Hábitos de Notificação e Barreiras Institucionais', fontsize=11, fontweight='bold', pad=15, color='#0f172a')
plt.tight_layout()
plt.savefig('docs/charts/03_canais_reclamacao.png')
plt.close()

# 4. Gráfico Aderência ZelaJampa
c_ade = Counter(aderencia)
ade_labels = ['Sim, com certeza usaria', 'Talvez (se for fácil)', 'Não usaria']
ade_keys = {
    'Sim, com certeza usaria': 'Sim com certeza',
    'Talvez (se for fácil)': 'Talvez facilidade',
    'Não usaria': 'Nao usaria'
}
counts_ade = [c_ade[ade_keys[k]] for k in ade_labels]

fig, ax = plt.subplots(figsize=(7, 3.8), dpi=300)
colors_ade = ['#059669', '#f59e0b', '#ef4444']
bars = ax.bar(ade_labels, counts_ade, color=colors_ade, width=0.5, edgecolor='none')

for bar in bars:
    yval = bar.get_height()
    pct = (yval / len(locomocao)) * 100
    ax.text(bar.get_x() + bar.get_width()/2, yval + 0.3, f'{yval} ({pct:.0f}%)', 
            ha='center', va='bottom', fontsize=9, fontweight='bold', color='#1e293b')

ax.set_title('4. Disposição para Usar Plataforma Web com Mapa e GPS', fontsize=12, fontweight='bold', pad=15, color='#0f172a')
ax.set_ylabel('Total de Respondentes', fontsize=10, color='#475569')
ax.set_ylim(0, max(counts_ade) + 2.5)
ax.grid(axis='y', linestyle='--', alpha=0.6)
ax.grid(axis='x', visible=False)
plt.tight_layout()
plt.savefig('docs/charts/04_adesao_zelajampa.png')
plt.close()

print('Gráficos gerados com sucesso em docs/charts/')
